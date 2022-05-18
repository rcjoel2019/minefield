use colored::Colorize;


struct Vector2{
    x:i32,
    y:i32
}


struct Game{
    grid: Vec<Vec<u32>>,
    visible: Vec<Vec<bool>>,
    flags: Vec<Vec<bool>>,
    size: Vector2,
    bomb_count: i32,
    open: bool,
}

impl Game{
    fn init_grid(&mut self, size:Vector2){
        self.grid = vec![vec![0;size.x as usize];size.y as usize];
        self.visible = vec![vec![false;size.x as usize];size.y as usize];
        self.flags = vec![vec![false;size.x as usize];size.y as usize];
        self.size = size;

        while self.bomb_count >= 1 {
            self.bomb_count -= 1;
            self.plant_bombs();
        }

        self.map_bombs();
        self.print_grid();

    }

    fn get_index(&self, input: String) -> i32{
        let yp:Vec<&str> = "A B C D E F G H I J K L M N O P Q R S T U V W X Y Z".split(" ").collect();
        let mut i = 0;
        for l in yp{
            if l.to_string() == input{
                return i;
            }
            i+=1;
        }
        return -1;
    }

    

    fn print_grid(&mut self){
        print!("{}[2J", 27 as char);
        let yp:Vec<&str> = "A B C D E F G H I J K L M N O P Q R S T U V W X Y Z".split(" ").collect();
        let mut yc:u32 = 0;
        print!("{} {} ", "Y".green(), "X".red());
        for i in 0..self.grid.get(0).unwrap().len(){
            print!("{} ", yp[i].to_string().red());
        }
        println!("");
        print!("~ ");print!("~ ");
        for _i in 0..self.grid.get(0).unwrap().len(){
            print!("~ ")
        }
        println!("");

        for y in 0..self.size.y{
            print!("{} | ",yp.get(yc as usize).unwrap().green());
            for x in 0..self.size.x{
                if self.visible[y as usize][x as usize] {
                    self.print_by_color(self.grid[y as usize][x as usize]);
                }else if self.flags[y as usize][x as usize]{
                    print!("{} ", "F".truecolor(255, 0, 0).bold());
                }
                else{
                    print!("{}","X ".truecolor(120, 120, 120));
                }
            }
            println!("");
            yc+=1;
        }

        if !self.open {
            println!("You lost the game !!!");
        }else if self.count_unmarked_bombs() == 0{
            println!("You win the game !!!");
            self.open = false;
        }
        
    }

    fn print_by_color(&mut self,num:u32){
        if num ==9{
            self.open = false;
        }
        match num {
            0 => print!("{} ", "-".black().bold()),//clear field
            1 => print!("{} ", "1".truecolor(0, 0, 255).bold()), //1 bomb nearby
            2 => print!("{} ", "2".truecolor(0, 255, 0).bold()), //2 bomb nearby
            3 => print!("{} ", "3".red().bold()), //3 bomb nearby
            4 => print!("{} ", "4".truecolor(5, 0, 92).bold()), //4 bomb nearby
            5 => print!("{} ", "5".truecolor(76, 0, 227).bold()), //5 bomb nearby
            6 => print!("{} ", "6".truecolor(227, 98, 0).bold()), //6 bomb nearby
            7 => print!("{} ", "7".truecolor(247, 227, 0).bold()), //7 bomb nearby
            8 => print!("{} ", "8".truecolor(231, 0, 247).bold()), //8 bomb nearby
            9 => print!("{} ", "B".truecolor(81, 16, 145).bold()), //bomb :(
            _ => print!("{} ", "E".purple().bold())
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

    fn reveal_nearby(&mut self, pos:Vector2){
        let consult = [
                                    Vector2{x:-1,y:-1},Vector2{x:0,y:-1},Vector2{x:1,y:-1},
                                    Vector2{x:-1,y:0}                   ,Vector2{x:1,y:0},
                                    Vector2{x:-1,y:1},Vector2{x:0,y:1},Vector2{x:1,y:1},
                                ];
        for c in consult{
            let cx = pos.x+c.x;
            let cy = pos.y+c.y;
            if pos.x+c.x >= 0 && pos.y+c.y >= 0 && pos.x+c.x < self.size.x && pos.y+c.y < self.size.y {
                if self.flags[cy as usize][cx as usize] == false && self.visible[cy as usize][cx as usize] == false {
                    self.visible[cy as usize][cx as usize] = true;
                }
            }
        }
            
    }

    fn show_field(&mut self,pos:Vector2){
        if self.grid[pos.y as usize][pos.x as usize] == 0{
            let consult = [
                                                    Vector2{x:-1,y:-1},Vector2{x:0,y:-1},Vector2{x:1,y:-1},
                                                    Vector2{x:-1,y:0},Vector2{x:0,y:0},Vector2{x:1,y:0},
                                                    Vector2{x:-1,y:1},Vector2{x:0,y:1},Vector2{x:1,y:1},
                                                    ];
            for c in consult{
                let cx = pos.x+c.x;
                let cy = pos.y+c.y;
                if pos.x+c.x >= 0 && pos.y+c.y >= 0 && pos.x+c.x < self.size.x && pos.y+c.y < self.size.y {
                    self.visible[cy as usize][cx as usize] = true;
                }
            }
        }else{
            self.visible[pos.y as usize][pos.x as usize] = true;
        }
        self.verify_vis_open_field();
    }

    fn verify_vis_open_field(&mut self){
        let mut has_error = false;
        for y in 0..self.size.y{
            for x in 0..self.size.x{
                let consult = [
                                            Vector2{x:-1,y:-1},Vector2{x:0,y:-1},Vector2{x:1,y:-1},
                                            Vector2{x:-1,y:0},Vector2{x:0,y:0},Vector2{x:1,y:0},
                                            Vector2{x:-1,y:1},Vector2{x:0,y:1},Vector2{x:1,y:1},
                                        ];
                for c in consult{
                    let cx = x+c.x;
                    let cy = y+c.y;
                    if x+c.x >= 0 && y+c.y >= 0 && x+c.x < self.size.x && y+c.y < self.size.y {
                        if self.visible[cy as usize][cx as usize] == false && self.visible[y as usize][x as usize] == true && self.grid[y as usize][x as usize] == 0 && self.grid[cy as usize][cx as usize] != 9{
                            self.visible[cy as usize][cx as usize] = true;
                            has_error = true;
                        }
                    }
                }
            }
        }
        if has_error{
            self.verify_vis_open_field();
        }
    }

    fn listen_action(&mut self){
        print!("Unmarked {} Enter: ", self.count_flags());
        println!("X, Y and ACTION(R: reveal, F: flag)");
        let com = self.read();
        if com.action == String::from("R"){
            if self.visible[com.y as usize][com.x as usize]{
                self.reveal_nearby(Vector2{x:com.x,y:com.y})
            }else{
                self.show_field(Vector2{x:com.x,y:com.y});
            }
        }else if com.action == String::from("F") {
            if !self.visible[com.y as usize][com.x as usize]{
                self.flags[com.y as usize][com.x as usize] = true;       
            }
        }else{
            self.listen_action()
        }
    }

    fn count_unmarked_bombs(&self)->u32{
        let mut unmarked = 0;
        for y in 0..self.size.y{
            for x in 0..self.size.x{
                if self.grid[y as usize][x as usize] == 9 && self.flags[y as usize][x as usize] == false {
                    unmarked += 1;
                }
            }
        }
        unmarked
    }

    fn count_flags(&self)->u32{
        let mut bombs = 0;
        let mut flags = 0;
        for y in 0..self.size.y{
            for x in 0..self.size.x{
                if self.grid[y as usize][x as usize] == 9 {
                    bombs += 1;
                }
                if self.flags[y as usize][x as usize]{
                    flags += 1;
                }
            }
        }
        bombs-flags
    }
    fn read(&self)-> Command{
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer);
        let str =buffer.trim_end().to_string();
        let res:Vec<&str> = str.split(" ").collect();
        Command{
            x: self.get_index(res[0].to_string().to_uppercase()),
            y: self.get_index(res[1].to_string().to_uppercase()),
            action: res[2].to_string().to_uppercase()
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

struct Command{
    x: i32,
    y: i32,
    action: String
}


fn read_size()-> Vector2{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    let str =buffer.trim_end().to_string();
    let res:Vec<&str> = str.split(" ").collect();
    Vector2{
        x: res[0].parse().unwrap(),
        y: res[1].parse().unwrap(),
    }
}

fn main() {
    println!("Insira o tamanho do campo ex: '10 10'");
    let mut size: Vector2 = read_size();
    if size.x > 26 {
        size.x = 26
    }
    if size.y > 26 {
        size.y = 26
    }
    let mut game: Game = Game{grid: vec![vec![0;0];0],visible: vec![vec![false;0];0], flags: vec![vec![false;0];0], size: Vector2 { x: 0, y: 0 }, bomb_count:( (size.x as f32 * size.y as f32) * 0.10) as i32, open: true};
    game.init_grid(size);

    while game.open {
        game.listen_action();
        game.print_grid();
    }
}
