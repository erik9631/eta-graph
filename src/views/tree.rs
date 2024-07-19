use crate::graph::{EdgeData, Header, MSize, Vertices};
pub struct TreeView<'a, T> {
    pub nodes: &'a mut EdgeData,
    pub values: &'a mut Vertices<T>,
}



impl <'a, T> TreeView<'a, T> {
    #[cfg_attr(release, inline(always))]
    pub fn new(edges: &'a mut EdgeData, vertices: &'a mut Vertices<T>) -> Self {
        return TreeView{
            nodes: edges,
            values: vertices,
        }
    }

    #[cfg_attr(release, inline(always))]
    pub fn add_child(&mut self, parent: MSize, child: MSize){
        self.nodes.connect(parent, child);
        self.nodes.set(child, parent, 1);
        self.nodes.set(child, self.get_root(parent), 0);
    }

    fn create_vertex(&mut self, val: T) -> MSize {
        self.values.push(val);
        self.nodes.create_vertex(0);
        let vertex = self.values.len() -1;
        return vertex as MSize;
    }
    #[cfg_attr(release, inline(always))]

    pub fn get_root(&self, vertex: MSize) -> MSize{
        return self.nodes.get(vertex, 0);
    }
    #[cfg_attr(release, inline(always))]

    pub fn get_parent(&self, vertex: MSize) -> MSize{
        return self.nodes.get(vertex, 1);
    }

    pub fn create_node(&mut self, val: T) -> MSize {
        let vertex = self.create_vertex(val);

        self.nodes.connect(vertex, vertex); // root
        self.nodes.connect(vertex, EdgeData::NONE); // parent

        return vertex;
    }

    #[cfg_attr(release, inline(always))]

    pub fn create_child(&mut self, parent: MSize, val: T) -> MSize {
        let child = self.create_vertex(val);
        self.add_child(parent, child);
        return child;
    }
}