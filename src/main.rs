use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    env, fs::File, io::{BufWriter, Write},
};

fn spice_to_mtx(path: &str) {
    let target_path = format!("{}.mtx", path);
    let spice_file = std::fs::read_to_string(path).unwrap();
    let mut nodes: HashMap<String, usize> = HashMap::new();
    let mut edges: Vec<(usize, usize, f64)> = vec![];
    for l in spice_file.lines().filter(|l| l.starts_with('R')) {
        let subs: Vec<_> = l.split(' ').collect();
        let f = subs[1];
        let t = subs[2];
        let w = subs[3].parse::<f64>().unwrap();
        let mut get_or_insert_id = |name: &str| {
            let id = nodes.len();
            match nodes.entry(name.to_string()) {
                Occupied(o) => *o.get(),
                Vacant(v) => {
                    v.insert(id);
                    id
                }
            }
        };
        let f = get_or_insert_id(f);
        let t = get_or_insert_id(t);
        edges.push((f, t, w));
    }
    let out = File::create(target_path).unwrap();
    let mut writer = BufWriter::new(out);
    writer.write_all(format!("{} {} {}\n", nodes.len(), nodes.len(), edges.len() * 2).as_bytes()).unwrap();
    for (f, t, w) in edges {
        writer.write_all(format!("{f} {t} {w}\n").as_bytes()).unwrap();
        writer.write_all(format!("{t} {f} {w}\n").as_bytes()).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in args.iter().skip(1) {
        spice_to_mtx(i);
    }
}
