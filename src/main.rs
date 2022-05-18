
struct Vector2{
    x:i32,
    y:i32
}

impl Vector2 {
    fn get_x(self)->i32{
        self.x
    }
}


struct Game{
    grid: Vec<Vec<u32>>,
    visible: Vec<Vec<bool>>,
    size: Vector2,
    bomb_count: u32
}

impl Game{
    fn init_grid(&mut self, size:Vector2){
        self.grid = vec![vec![0;size.x as usize];size.y as usize];
        self.visible = vec![vec![false;size.x as usize];size.y as usize];
        self.size = size;

        while self.bomb_count > 1 {
            self.bomb_count -= 1;
            self.plant_bombs();
        }

        self.map_bombs();
        self.print_grid();

    }

    fn print_grid(&self){
        let yp:Vec<&str> = "A B C D E F G H I J K L M N O P Q R S T U V W X Y Z".split(" ").collect();
        let mut yc:u32 = 0;
        print!("Y X ");
        for i in 0..self.grid.get(0).unwrap().len(){
            print!("{} ", i);
        }
        println!("");
        print!("- ");print!("- ");
        for _i in 0..self.grid.get(0).unwrap().len(){
            print!("- ")
        }
        println!("");

        for y in 0..self.size.y{
            print!("{} | ",yp.get(yc as usize).unwrap());
            for x in 0..self.size.x{
                if(self.visible[y as usize][x as usize]){
                    print!("{} ", self.grid[y as usize][x as usize]);
                }else{
                    print!("X ");
                }
            }
            println!("");
            yc+=1;
        }

        
    }
    //9 is a bomb
    fn plant_bombs(&mut self){
        let x = self.size.x as f64 * rand::random::<f64>();
        let y = self.size.y as f64 * rand::random::<f64>();
        if self.grid[y as usize][x as usize] == 9 {
            self.plant_bombs()
        }else{
            self.grid[y as usize][x as usize] = 9;
        }
    }

    fn map_bombs(&mut self){
        for y in 0..self.size.y{
            for x in 0..self.size.x{

                if self.grid[y as usize][x as usize] != 9{
                    let consult = [
                                                    Vector2{x:-1,y:-1},Vector2{x:0,y:-1},Vector2{x:1,y:-1},
                                                    Vector2{x:-1,y:0},Vector2{x:0,y:0},Vector2{x:1,y:0},
                                                    Vector2{x:-1,y:1},Vector2{x:0,y:1},Vector2{x:1,y:1},
                                                    ];
                    let mut bombs:u32 = 0;
                    for c in consult{
                        let cx = x+c.x;
                        let cy = y+c.y;
    
                        if x+c.x >= 0 && y+c.y >= 0 && x+c.x < self.size.x && y+c.y < self.size.y {
                            if self.grid[cy as usize][cx as usize] == 9{
                                bombs += 1;
                            }
                        }
                    }
                    self.grid[y as usize][x as usize] = bombs;
                }

                
            }
        }
    }

}

fn main() {
    const SIZE: Vector2 = Vector2{x:10,y:10};
    let mut game: Game = Game{grid: vec![vec![0;0];0],visible: vec![vec![false;0];0], size: Vector2 { x: 0, y: 0 }, bomb_count: 7};
    game.init_grid(SIZE);
}
