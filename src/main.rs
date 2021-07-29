/*-----------------------------------------work 1----------------------------------------------*/
enum Light{
    Red,
    Yellow,
    Green,
} 
pub trait SinceTime{
    fn sintime(&self);
}

impl SinceTime for Light{
    fn sintime(&self){
        match self{
            Light::Red => println!("60s"),
            Light::Yellow => println!("40s"),
            Light::Green => println!("120s"),
        }
    }
}
/*---------------------------------------------------------------------------------------------*/


/*-----------------------------------------work 2----------------------------------------------*/
fn SumUint(nums:&[u32]) -> Option<u32>{
    let mut sum :u32 = 0;
    for i in nums{
        sum += i;
        if (sum > std::u32::MAX){
            return None
        }else{
            continue;
        };
    }
    Some(sum)
}
/*---------------------------------------------------------------------------------------------*/



/*-----------------------------------------work 2----------------------------------------------*/
struct Triangle {
    width: f32,
    height: f32,
}

struct Circular {
    radius: f32,
}

struct Square {
    len : f32,
    wid : f32,
}

trait ComputeArea {
    fn compute(&self) -> f32;
}

impl ComputeArea for Triangle{
    fn compute(&self) -> f32{
        (self.width * self.height)/2.0
    }
}

impl ComputeArea for Circular{
    fn compute(&self) -> f32{
        3.14 * self.radius.powf(2.0)
    }
}

impl ComputeArea for Square{
    fn compute(&self) -> f32{
        self.len * self.wid
    }
}

fn ComputeSomeAre<T:ComputeArea>(k:T)->f32{
    k.compute()
}

/*---------------------------------------------------------------------------------------------*/
fn main() {

}

