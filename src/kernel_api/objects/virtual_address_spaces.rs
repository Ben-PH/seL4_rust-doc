//! Corresponding to hardware memory.



pub struct PageGlobalDirectory{}

impl PageGlobalDirectory {
    fn clean_data(){}
    fn invalidate_data(){}
    fn clean_invalidate_data(){}
    fn unify_instruction(){}

}

pub struct PageUpperDirectory{}
impl PageUpperDirectory{
    pub fn map(){}
    pub fn unmap(){}
}
pub struct PageDirectory{}
impl PageDirectory{
    pub fn map(){}
    pub fn unmap(){}
}
pub struct PageTable{}
impl PageTable{
    pub fn map(){}
    pub fn unmap(){}
}

pub struct IOPageTable{}
impl IOPageTable{
    pub fn map(){}
    pub fn unmap(){}
}

pub struct Page {}

impl Page{
    pub fn map(){}
    pub fn unmap(){}
    pub fn remap(){}
    pub fn clean_data(){}
    pub fn invalidate_data(){}
    pub fn clean_invalidate_data(){}
    pub fn unify(){}
}


pub struct ASIDPool{}
impl ASIDPool {
    pub fn assign(){}
}
