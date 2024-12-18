//! Utilities for working with graphs of things.

pub mod traits {
    pub mod graph {
        use std::hash::Hash;

        pub trait Graph {
            /// The type of vertex in the graph.
            type Vertex: Copy + Eq + Hash;

            /// The type of edge in the graph.
            type Edge: Copy + Eq + Hash;
            // type Edge = (Self::Node, Self::Node);
        }

        /// A graph with efficient access to the out-edges of each vertex.
        pub trait IncidenceGraph: Graph {
            /// Returns the outgoing edges from a vertex.
            ///
            /// # Complexity
            /// This must be constant time.
            ///
            /// # TODO
            /// - Should this be constrained to return an iterator that is double-ended or exact-sized?
            fn out_edges(&self, vertex: Self::Vertex) -> impl Iterator<Item = Self::Edge>;

            /// Returns the source vertex of an edge.
            ///
            /// # Complexity
            /// This must be constant time.
            fn source(&self, edge: Self::Edge) -> Self::Vertex;

            /// Returns the target vertex of an edge.
            ///
            /// # Complexity
            /// This must be constant time.
            fn target(&self, edge: Self::Edge) -> Self::Vertex;

            /// Returns the number of out-edges from a vertex.
            ///
            /// This has a default implementation that is the length of the iterator returned by [`IncidenceGraph::out_edges`].
            ///
            /// # Complexity
            /// This must be linear in time in the number of out-edges of the vertex, but may be faster in some cases.
            fn out_degree(&self, vertex: Self::Vertex) -> usize {
                self.out_edges(vertex).count()
            }
        }

        /// A graph with efficient access to the in-edges of each vertex.
        pub trait BidirectionalGraph: IncidenceGraph {
            /// Returns the incoming edges to a vertex.
            ///
            /// # Complexity
            /// This must be constant time.
            ///
            /// # TODO
            /// - Should this be constrained to return an iterator that is double-ended or exact-sized?
            fn in_edges(&self, vertex: Self::Vertex) -> impl Iterator<Item = Self::Edge>;

            /// Returns the number of in-edges to a vertex.
            ///
            /// This has a default implementation that is the length of the iterator returned by [`BidirectionalGraph::in_edges`].
            ///
            /// # Complexity
            /// This must be linear in time in the number of in-edges of the vertex, but may be faster in some cases.
            fn in_degree(&self, vertex: Self::Vertex) -> usize {
                self.in_edges(vertex).count()
            }

            /// Returns the number of in-edges plus out-edges to a vertex.
            ///
            /// This has a default implementation that is the sum of the in-degree and out-degree.
            /// Other implementations may override this to provide a more efficient implementation.
            ///
            /// # Complexity
            /// This must be linear in time in the number of in-edges and out-edges of the vertex, but may be faster in some cases.
            fn degree(&self, vertex: Self::Vertex) -> usize {
                self.in_degree(vertex) + self.out_degree(vertex)
            }
        }

        /// A graph with efficient access to the adjacenct vertices of a vertex.
        ///
        /// This is a more relaxed version of [`IncidenceGraph`] where only vertices are important.
        pub trait AdjacencyGraph: Graph {
            /// Returns the adjacent vertices of a vertex.
            ///
            /// # Complexity
            /// This must be constant time.
            ///
            /// # TODO
            /// - Should this be constrained to return an iterator that is double-ended or exact-sized?
            fn adjacenct_vertices(
                &self,
                vertex: Self::Vertex,
            ) -> impl Iterator<Item = Self::Vertex>;
        }

        /// A graph with efficient traversal of all vertices.
        pub trait VertexListGraph: Graph {
            /// Returns the number of vertices in the graph.
            ///
            /// The default implementation utilizes the [`EdgeListGraph::edges`] method since it returns an [`ExactSizeIterator`].
            ///
            /// # Complexity
            /// This must be constant time.
            fn num_vertices(&self) -> usize {
                self.vertices().len()
            }

            /// Returns the vertices of the graph.
            ///
            /// # Complexity
            /// This must be constant time.
            fn vertices(&self) -> impl ExactSizeIterator<Item = Self::Vertex>;
        }

        /// A graph with efficient traversal of all edges.
        pub trait EdgeListGraph: Graph {
            /// Returns the edges of the graph.
            ///
            /// # Complexity
            /// This must be constant time.
            fn edges(&self) -> impl ExactSizeIterator<Item = Self::Edge>;

            /// Returns the number of edges in the graph.
            ///
            /// The default implementation utilizes the [`EdgeListGraph::edges`] method since it returns an [`ExactSizeIterator`].
            ///
            /// # Complexity
            /// This must be constant time.
            fn num_edges(&self) -> usize {
                self.edges().len()
            }

            /// Returns the source of an edge.
            ///
            /// # Complexity
            /// This must be constant time.
            fn source(&self, edge: Self::Edge) -> Self::Vertex;

            /// Returns the target of an edge.
            ///
            /// # Complexity
            /// This must be constant time.
            fn target(&self, edge: Self::Edge) -> Self::Vertex;
        }

        /// A graph with efficient traversal of all vertices and edges.
        pub trait VertexAndEdgeListGraph: VertexListGraph + EdgeListGraph {}

        /// A graph with efficient access to any edge from a source and target vertex.
        pub trait AdjacencyMatrix: Graph {
            /// Returns the edge from a source vertex to a target vertex.
            ///
            /// # Complexity
            /// This must be constant time.
            fn edge(&self, source: Self::Vertex, target: Self::Vertex) -> Option<Self::Edge>;
        }

        /// A graph which can be changed with the addition and removal of vertices and edges.
        pub trait MutableGraph: Graph {
            /// Adds a vertex to the graph.
            ///
            /// # Complexity
            /// This must be amortized constant time.
            fn add_vertex(&mut self) -> Self::Vertex;

            /// Removes all edges to and from a vertex.
            /// Note: this does not remove the vertex itself.
            ///
            /// # Complexity
            /// This must be linear in time of the number of edges and vertices in the graph: `O(E + V)`.
            fn clear_vertex(&mut self, vertex: Self::Vertex);

            /// Removes a vertex from the graph.
            /// Note: this also clears all edges to and from the vertex. See [`MutableGraph::clear_vertex`].
            ///
            /// # Complexity
            /// This must be linear in time of the number of edges and vertices in the graph: `O(E + V)`.
            fn remove_vertex(&mut self, vertex: Self::Vertex);

            /// Adds an edge to the graph.
            ///
            /// # Complexity
            /// If parallel edges are allowed, this must be `O(log(E/V))`.
            /// Otherwise, it must be amortized constant.
            ///
            /// # Returns
            /// The edge that was added. If parallel edges are not allowed, and an edge already exists between the source and
            /// target vertices, that edge should be returned.
            fn add_edge(&mut self, source: Self::Vertex, target: Self::Vertex) -> Self::Edge;

            /// Removes an edge from the graph.
            ///
            /// # Complexity
            /// This must be linear in time of the number of edges in the graph: `O(E)`.
            fn remove_edge(&mut self, edge: Self::Edge);

            /// Removes all edges between two vertices.
            ///
            /// # Complexity
            /// This must be linear in time of the number of edges in the graph: `O(E)`.
            fn remove_edge_between(&mut self, source: Self::Vertex, target: Self::Vertex);
        }
    }

    pub mod map {
        //! A map from vertices to values.

        use super::graph::Graph;
        use core::hash::Hash;
        use std::borrow::Borrow;
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        /// A read-only map from vertices to values.
        pub trait ReadMap<G: Graph, V> {
            /// Returns a reference to the value corresponding to the key.
            ///
            /// The key may be any borrowed form of the map's key type, but
            /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
            /// the key type.
            fn get<Q>(&self, vertex: &Q) -> Option<&V>
            where
                G::Vertex: Borrow<Q>,
                Q: ?Sized + Hash + Eq;
        }

        /// A write-only map from vertices to values.
        pub trait WriteMap<G: Graph, V> {
            /// Inserts a key-value pair into the map.
            ///
            /// If the map did not have this key present, [`None`] is returned.
            ///
            /// If the map did have this key present, the value is updated, and the old value is returned.
            fn insert(&mut self, vertex: G::Vertex, value: V) -> Option<V>;
        }

        /// A readable and writable map from vertices to values.
        pub trait ReadWriteMap<G: Graph, V>: ReadMap<G, V> + WriteMap<G, V> {
            fn entry(&mut self, vertex: G::Vertex) -> Entry<'_, G::Vertex, V>;
        }

        impl<G: Graph, V> ReadMap<G, V> for HashMap<G::Vertex, V> {
            fn get<Q>(&self, vertex: &Q) -> Option<&V>
            where
                G::Vertex: Borrow<Q>,
                Q: ?Sized + Hash + Eq,
            {
                self.get(vertex)
            }
        }

        impl<G: Graph, V> WriteMap<G, V> for HashMap<G::Vertex, V> {
            fn insert(&mut self, vertex: G::Vertex, value: V) -> Option<V> {
                self.insert(vertex, value)
            }
        }

        impl<G: Graph, V> ReadWriteMap<G, V> for HashMap<G::Vertex, V> {
            fn entry(&mut self, vertex: G::Vertex) -> Entry<'_, G::Vertex, V> {
                self.entry(vertex)
            }
        }
    }
}

pub mod algorithms {
    use std::borrow::Borrow;
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;

    use crate::graph::traits::graph::{Graph, IncidenceGraph, VertexListGraph};
    use crate::graph::traits::map::{ReadMap, ReadWriteMap, WriteMap};

    use crate::graph::visitors::{BreadthFirstSearchVisitor, DepthFirstSearchVisitor};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Color {
        /// The vertex has not been visited yet.
        White,

        /// The vertex has been visited, but not all of its neighbors have been visited.
        Gray,

        /// The vertex has been visited, and all of its neighbors have been visited.
        Black,
    }

    /// Performs a breadth-first search on a graph.
    pub fn breadth_first_search<G>(
        graph: &G,
        start: G::Vertex,
        visitor: &mut impl BreadthFirstSearchVisitor<G>,
    ) where
        G: VertexListGraph + IncidenceGraph,
    {
        let mut colors = HashMap::new();
        for vertex in graph.vertices() {
            visitor.initialize_vertex(graph, vertex);
            colors.insert(vertex, Color::White);
        }

        breadth_first_visit(graph, start, visitor, &mut colors)
    }

    /// Performs a breadth-first search on a graph.
    ///
    /// This is similar to [`breadth_first_search`], but it does not perform discovery over the vertices in the graph.
    /// As such, the graph is not required to implement [`VertexListGraph`].
    /// Additionally, the caller must provide a map of colors for each vertex in the graph.
    pub fn breadth_first_visit<G>(
        graph: &G,
        start: G::Vertex,
        visitor: &mut impl BreadthFirstSearchVisitor<G>,
        colors: &mut impl ReadWriteMap<G, Color>,
    ) where
        G: IncidenceGraph,
    {
        let mut queue = VecDeque::new();

        visitor.discover_vertex(graph, start);
        colors.insert(start, Color::Gray);

        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            visitor.examine_vertex(graph, u);
            for edge in graph.out_edges(u) {
                visitor.examine_edge(graph, edge);
                let v = graph.target(edge);
                let color = colors.entry(v).or_insert(Color::White);
                if *color == Color::White {
                    visitor.discover_vertex(graph, v);
                    visitor.tree_edge(graph, edge);
                    *color = Color::Gray;
                    queue.push_back(v);
                } else {
                    visitor.non_tree_edge(graph, edge);
                    if *color == Color::Gray {
                        visitor.gray_target(graph, edge);
                    } else {
                        visitor.black_target(graph, edge);
                    }
                }
            }

            visitor.finish_vertex(graph, u);
            colors.entry(u).and_modify(|c| *c = Color::Black);
        }
    }

    /// Performs a depth-first search on a graph.
    ///
    /// If `start` is provided, the search will start at that vertex.
    /// After the search is complete, the search will continue from any vertices that have not been visited.
    ///
    /// To only search a single connected component, use [`depth_first_visit`] instead.
    pub fn depth_first_search<G>(
        graph: &G,
        start: Option<G::Vertex>,
        visitor: &mut impl DepthFirstSearchVisitor<G>,
    ) where
        G: VertexListGraph + IncidenceGraph,
    {
        let mut colors = HashMap::new();
        for vertex in graph.vertices() {
            visitor.initialize_vertex(graph, vertex);
            colors.insert(vertex, Color::White);
        }

        if let Some(start) = start {
            visitor.start_vertex(graph, start);
            depth_first_visit(graph, start, visitor, &mut colors);
        }

        for vertex in graph.vertices() {
            if colors[&vertex] == Color::White {
                visitor.start_vertex(graph, vertex);
                depth_first_visit(graph, vertex, visitor, &mut colors);
            }
        }
    }

    /// Visits vertices rooted at `start` in a depth-first search.
    ///
    /// This is similar to [`depth_first_search`], but it does not perform initialization over the vertices in the graph.
    /// As such, the graph is not required to implement [`VertexListGraph`].
    ///
    /// Additionally, the caller must provide a map of colors for each vertex in the graph.
    /// If a vertex is not in the map, it is assumed to be white.
    pub fn depth_first_visit<G>(
        graph: &G,
        start: G::Vertex,
        visitor: &mut impl DepthFirstSearchVisitor<G>,
        colors: &mut impl ReadWriteMap<G, Color>,
    ) where
        G: IncidenceGraph,
    {
        let u = start;
        visitor.discover_vertex(graph, u);
        colors.insert(start, Color::Gray);
        for edge in graph.out_edges(u) {
            visitor.examine_edge(graph, edge);
            let v = graph.target(edge);
            let color = colors.entry(v).or_insert(Color::White);
            if *color == Color::White {
                visitor.tree_edge(graph, edge);
                depth_first_visit(graph, v, visitor, colors);
            } else if *color == Color::Gray {
                visitor.back_edge(graph, edge);
            } else {
                visitor.forward_or_cross_edge(graph, edge);
            }

            visitor.finish_edge(graph, edge);
        }

        visitor.finish_vertex(graph, u);
        colors.insert(u, Color::Black);
    }

    /// Records the predecessor of each vertex in a search.
    pub struct Predecessors<G: Graph>(HashMap<G::Vertex, G::Vertex>);

    impl<G: Graph> Predecessors<G> {
        pub fn new() -> Self {
            Self(HashMap::new())
        }
    }

    impl<G: Graph> ReadMap<G, G::Vertex> for Predecessors<G> {
        fn get<Q>(&self, vertex: &Q) -> Option<&G::Vertex>
        where
            G::Vertex: Borrow<Q>,
            Q: ?Sized + Hash + Eq,
        {
            self.0.get(vertex)
        }
    }

    impl<G: Graph> WriteMap<G, G::Vertex> for Predecessors<G> {
        fn insert(&mut self, vertex: G::Vertex, value: G::Vertex) -> Option<G::Vertex> {
            self.0.insert(vertex, value)
        }
    }

    impl<G: Graph> ReadWriteMap<G, G::Vertex> for Predecessors<G> {
        fn entry(
            &mut self,
            vertex: G::Vertex,
        ) -> std::collections::hash_map::Entry<'_, G::Vertex, G::Vertex> {
            self.0.entry(vertex)
        }
    }

    impl<G: IncidenceGraph> BreadthFirstSearchVisitor<G> for Predecessors<G> {
        fn tree_edge(&mut self, graph: &G, edge: G::Edge) {
            let u = graph.source(edge);
            let v = graph.target(edge);
            self.insert(v, u);
        }
    }

    impl<G: IncidenceGraph> DepthFirstSearchVisitor<G> for Predecessors<G> {
        fn tree_edge(&mut self, graph: &G, edge: <G as Graph>::Edge) {
            let u = graph.source(edge);
            let v = graph.target(edge);
            self.insert(v, u);
        }
    }

    /// Records the distance and predecessor of each vertex in a search.
    pub struct DistanceAndPredecessor<G: Graph> {
        distances: HashMap<G::Vertex, usize>,
        predecessors: Predecessors<G>,
    }

    impl<G: Graph> DistanceAndPredecessor<G> {
        pub fn new() -> Self {
            Self {
                distances: HashMap::new(),
                predecessors: Predecessors::new(),
            }
        }
    }

    impl<G: IncidenceGraph> BreadthFirstSearchVisitor<G> for DistanceAndPredecessor<G> {
        fn discover_vertex(&mut self, graph: &G, vertex: G::Vertex) {
            self.distances.insert(vertex, 0);
            BreadthFirstSearchVisitor::discover_vertex(&mut self.predecessors, graph, vertex);
        }

        fn tree_edge(&mut self, graph: &G, edge: G::Edge) {
            let u = graph.source(edge);
            let v = graph.target(edge);
            self.distances.insert(v, self.distances[&u] + 1);
            BreadthFirstSearchVisitor::tree_edge(&mut self.predecessors, graph, edge);
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::{HashMap, HashSet};

        use crate::graph::algorithms::{DistanceAndPredecessor, Predecessors};
        use crate::graph::traits::graph::*;
        use crate::graph::traits::map::ReadMap;

        struct AdjacencyList {
            adjacencies: HashMap<usize, Vec<usize>>,
        }

        impl AdjacencyList {
            pub fn new(mut adjacencies: HashMap<usize, Vec<usize>>) -> Self {
                // there may be vertices in the adjacencies that are not keys
                let mut vertices = adjacencies.keys().copied().collect::<HashSet<_>>();
                vertices.extend(adjacencies.values().flat_map(|v| v.iter().copied()));
                for vertex in vertices {
                    adjacencies.entry(vertex).or_insert_with(Vec::new);
                }

                Self { adjacencies }
            }
        }

        impl Graph for AdjacencyList {
            type Vertex = usize;
            type Edge = (usize, usize);
        }

        impl IncidenceGraph for AdjacencyList {
            fn out_edges(&self, vertex: usize) -> impl Iterator<Item = (usize, usize)> {
                self.adjacencies
                    .get(&vertex)
                    .into_iter()
                    .flat_map(move |adjacencies| {
                        adjacencies.iter().copied().map(move |v| (vertex, v))
                    })
            }

            fn source(&self, edge: (usize, usize)) -> usize {
                edge.0
            }

            fn target(&self, edge: (usize, usize)) -> usize {
                edge.1
            }
        }

        impl VertexListGraph for AdjacencyList {
            fn vertices(&self) -> impl ExactSizeIterator<Item = usize> {
                0..self.adjacencies.len()
            }
        }

        #[test]
        fn breadth_first_search() {
            let graph = AdjacencyList::new(HashMap::from_iter([
                (0, vec![1, 2]),
                (1, vec![0, 3, 4]),
                (2, vec![0, 5]),
                (3, vec![1]),
                (4, vec![1]),
                (5, vec![2, 4]),
            ]));

            let mut visitor = DistanceAndPredecessor::new();
            super::breadth_first_search(&graph, 0, &mut visitor);

            assert_eq!(visitor.distances[&0], 0);
            assert_eq!(visitor.distances[&1], 1);
            assert_eq!(visitor.distances[&2], 1);
            assert_eq!(visitor.distances[&3], 2);
            assert_eq!(visitor.distances[&4], 2);
            assert_eq!(visitor.distances[&5], 2);

            assert_eq!(visitor.predecessors.get(&1), Some(&0));
            assert_eq!(visitor.predecessors.get(&2), Some(&0));
            assert_eq!(visitor.predecessors.get(&3), Some(&1));
            assert_eq!(visitor.predecessors.get(&4), Some(&1));
            assert_eq!(visitor.predecessors.get(&5), Some(&2));
        }

        #[test]
        fn depth_first_search() {
            let graph = AdjacencyList::new(HashMap::from_iter([
                (0, vec![1, 2]),
                (1, vec![0, 3, 4]),
                (2, vec![0, 5]),
                (3, vec![1]),
                (4, vec![1]),
                (5, vec![2, 4]),
            ]));

            let mut visitor = Predecessors::new();
            super::depth_first_search(&graph, Some(0), &mut visitor);

            assert_eq!(visitor.get(&1), Some(&0));
            assert_eq!(visitor.get(&2), Some(&0));
            assert_eq!(visitor.get(&3), Some(&1));
            assert_eq!(visitor.get(&4), Some(&1));
            assert_eq!(visitor.get(&5), Some(&2));
        }

        #[test]
        fn depth_first_search_two_components() {
            let graph = AdjacencyList::new(HashMap::from_iter([(0, vec![1]), (2, vec![3])]));

            let mut visitor = Predecessors::new();
            super::depth_first_search(&graph, Some(0), &mut visitor);

            assert_eq!(visitor.get(&0), None);
            assert_eq!(visitor.get(&1), Some(&0));
            assert_eq!(visitor.get(&2), None);
            assert_eq!(visitor.get(&3), Some(&2));
        }

        #[test]
        fn depth_first_visit_two_components() {
            let graph = AdjacencyList::new(HashMap::from_iter([(0, vec![1]), (2, vec![3])]));
            let mut visitor = Predecessors::new();
            let mut colors = HashMap::new();
            super::depth_first_visit(&graph, 0, &mut visitor, &mut colors);

            assert_eq!(visitor.get(&0), None);
            assert_eq!(visitor.get(&1), Some(&0));
            assert_eq!(colors.get(&2), None, "Vertex 2 should not be discovered");
            assert_eq!(colors.get(&3), None, "Vertex 3 should not be discovered");
            assert_eq!(visitor.get(&2), None);
            assert_eq!(visitor.get(&3), None);
        }
    }
}

pub mod visitors {
    use crate::graph::traits::graph::Graph;

    /// A visitor for breadth-first search.
    ///
    /// During the search, the visitor is invoked at various points in the search process.
    #[allow(unused_variables)]
    pub trait BreadthFirstSearchVisitor<G: Graph> {
        /// Invoked on every vertex of the graph before the start of the graph search.
        fn initialize_vertex(&mut self, graph: &G, vertex: G::Vertex) {}

        /// Invoked on every vertex of the graph after the vertex is discovered for the first time.
        fn discover_vertex(&mut self, graph: &G, vertex: G::Vertex) {}

        /// Invoked on every vertex as it is popped from the queue.
        ///
        /// This happens immediately before [`BreadthFirstSearchVisitor::examine_edge`] is called on each out-edge of the vertex.
        fn examine_vertex(&mut self, graph: &G, vertex: G::Vertex) {}

        /// Invoked on every out-edge of each vertex after it is discovered.
        fn examine_edge(&mut self, graph: &G, edge: G::Edge) {}

        /// Invoked on each edge as it becomes a member of the edges that form the search tree.
        ///
        /// This indicates the edge was examined and forms part of the search tree.
        fn tree_edge(&mut self, graph: &G, edge: G::Edge) {}

        /// Invoked on the back, or cross, edges in the graph.
        ///
        /// This indicates the edge was examined but does not form part of the search tree.
        fn non_tree_edge(&mut self, graph: &G, edge: G::Edge) {}

        /// Invoked on each edge whose target vertex is gray when examined.
        ///
        /// This indicates the target vertex is currently in the queue but has not been processed yet.
        fn gray_target(&mut self, graph: &G, edge: G::Edge) {}

        /// Invoked on each edge whose target vertex is black when examined.
        ///
        /// This indicates the target vertex has already been processed.
        fn black_target(&mut self, graph: &G, edge: G::Edge) {}

        /// Invoked on a vertex after all of its out-edges have been added to the queue and all adjacenct vertices are gray or black.
        fn finish_vertex(&mut self, graph: &G, vertex: G::Vertex) {}
    }

    /// A visitor for depth-first search.
    ///
    /// During the search, the visitor is invoked at various points in the search process.
    #[allow(unused_variables)]
    pub trait DepthFirstSearchVisitor<G: Graph> {
        /// Invoked on every vertex of the graph before the start of the graph search.
        fn initialize_vertex(&mut self, graph: &G, vertex: G::Vertex) {}

        /// Invoked on the source vertex once before the start of the search.
        fn start_vertex(&mut self, graph: &G, vertex: G::Vertex) {}

        /// Invoked when a vertex is encountered for the first time.
        fn discover_vertex(&mut self, graph: &G, vertex: G::Vertex) {}

        /// Invoked on every out-edge of each vertex after it is discovered.
        fn examine_edge(&mut self, graph: &G, edge: G::Edge) {}

        /// Invoked on each edge as it becomes a member of the edges that form the search tree.
        fn tree_edge(&mut self, graph: &G, edge: G::Edge) {}

        /// Invoked on the back edges of the graph.
        fn back_edge(&mut self, graph: &G, edge: G::Edge) {}

        /// Invoked on the forward or cross edges of the graph.
        fn forward_or_cross_edge(&mut self, graph: &G, edge: G::Edge) {}

        /// Invoked on each non-tree edge and on each tree edge after [`DepthFirstSearchVisitor::finish_vertex`] has been called on its target vertex.
        fn finish_edge(&mut self, graph: &G, edge: G::Edge) {}

        /// Invoked on each vertex after [`DepthFirstSearchVisitor::finish_edge`] has been called on for all vertices in the DFS tree rooted at the vertex.
        ///
        /// If the vertex is a leaf in the DFS tree, this will be called after all out-edges have been examined.
        fn finish_vertex(&mut self, graph: &G, vertex: G::Vertex) {}
    }
}
