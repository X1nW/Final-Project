use std::collections::HashMap;

pub struct GraphIterator<'a, T:Ord + Debug>{
    pub graph: &Graph<T>,
    pub stack:Vec<usize>
}

impl <'a, T:Ord + Debug> Iterator for GraphIterator<'a,T>{
    type Item = usize;

    fn next(&mut self) -> Option<self::Item>{
        if self.graph.is_empty(){
            return None;
        }else{
            while !self.stack.is_empty(){
                let self.stack.pop().unwrap();
                if self.visited[v]{
                    continue;
                }
                self.visited[v] = true;
                for e in self.graph.nei(v){
                    if !self.visted[e]{
                        self.stack.push(e)
                    }
                }
                return Some(v);
            }
            None
        }
    }
}

#[derive(Debug)]
pub struct Graph<T>{
    pub matrix:Vec<Vec<Option<usize>>>,
    pub nodes:HashMap<usize,Option<T>>,
}

impl<T: Debug + Ord> Graph <T> {
    pub fn new()->Self{
        Self { 
            matrix:vec![],
            nodes:HashMap::new()}
    }

    pub fn nei(&self,i:usize) -> Vec<usize>{
        let mut result = vec![];
        for j in 1..= self.bound(){
            if self.matrix[i][j].is_some(){
                result.push(j);
            }
        }
        result
    }

    pub fn is_empty(&self) -> bool{
        self.nodes.len() == 0
    }

    pub fn add_node(&mut self,index:usize) {
        self.nodes.insert(index,None);
        self.fix_length(index);
    }

    pub fn add_node_with_value(& mut self, index:usize, value: T) {
        self.nodes.insert(index,Some(value));
        self.fix_length(index);
    }

    pub fn fix_length(&mut self,index:usize) -> bool{
        if self.matrix.len() > index{
            false
        } else{
            let target_len = (index as f32*1.1) as usize + 2;
        
            while self.matrix.len() < target_len {
                self.matrix.push(vec![]);
            }
            for i in 0..target_len {
                while self.matrix[i].len() < target_len{
                    self.matrix[i].push(None)
                }
            }
            true
        }
    }

    pub fn add_edge(&mut self,from:usize,to:usize,weight:usize){
        if !self.nodes.contains_key(&from) {
            println!("from{}",from);
            self.add_node(from);
        }
        if !self.nodes.contains_key(&to) {
            println!("from{}",to);
            self.add_node(to);
        }
        self.matrix[from][to] = Some(weight);
    }

    pub fn debug(&self) -> String{
        let bound = self.bound();
        let mut paint = "  *".to_string();
        (1..=self.bound()).for_each(|x| paint.push_str(&format!("{:2} ",x)));
        paint.push_str("\n");
        for i in 1..=bound(){
            paint.push_str(&format!("{:2} ",i));
            for j in 1..=bound(){
                paint.push_str(&format!(
                    "{:2} ",
                    (match self.matrix[i][j]{
                        Some(e) => format!("{}",e),
                        None => String::from("."),
                    })
                ))
            }
            paint.push_str("\n");
        }
        paint
    }

    pub fn print(&self){
        println!("{}",self.debug())
    }

    pub fn remove_node(&mut self,index:usize) -> bool{
        if self.nodes.contains_key(&index){
            false
        }else{
            self.nodes.remove(&index);
            self.matrix[index].fill(None);
            for i in 1..=self.bound() {
                self.matrix[i][index] = None;
            }
            true
        }
    }

    pub fn remove_edge(&mut self,from:usize,to:usize) -> bool{
        if from.max(to) > self.bound() {
            false
        }else{
            self.matrix[from][to] = None;
            true
        }
    }

    pub fn bound(&self) -> usize{
        match self.nodes.iter().max() {
            Some((&e,_)) => e,
            None => 0,
        }
    }

    pub fn first(&self) -> Option<uszie>{
        match self.nodes.iter().map(|x| *x.0).collect::<Vec<usize>>().first(){
            Some(e) => Some(*e),
            None => None,
        }
    }

    pub fn dfs_iter(&self) -> GraphIterator<T> {
        let v = match self.get_first(){
            Some(e) => e,
            None => 0
        };
        GraphIterator { 
            graph: &self, 
            stack:vec![v], 
            visited:vec![false;self.bound()+1],
        }
    }

    pub fn into_iter() ->
}


#[test]
fn graph_test() {
    let mut g:Graph<String> = Graph::new();
    g.add_node(2);
    g.add_node(5);
    println!("{:#?}",g)
}