// init
if "myColor" in memory != true {
    memory["myColor"] = map[worker(0).x][worker(0).y];
}
let usedPositions = [];
for w in map.workers{
    usedPositions.push([w.x, w.y]);
}
fn MoveToDirection(workerNum, axis) {
    if axis == 0 {
        worker(workerNum).move_left();
    }
    else if axis == 1 {
        worker(workerNum).move_right();
    }
    else if axis == 2 {
        worker(workerNum).move_up();
    }
    else if axis == 3 {
        worker(workerNum).move_down();
    }
}
fn ConnectedComponents(x, y, map){
    let color = map[x][y];
    let m = [[x,y]];
    let visited = [[]];
    let n = 0;
    while m.len != 0{
        let element = m.pop();
        if !visited.contains(element) {
            n += 1;
            visited.push(element);
        let xl = element[0] - 1;
        let xr = element[0] + 1;
        let yu = element[1] - 1;
        let yd = element[1] + 1;
        if xl < 40 && xl >= 0 {
            if map[xl][element[1]]== color {
                m.push([xl,element[1]]);
            }
        }
        if xr < 40 && xr >= 0 {
            if map[xr][element[1]]== color {
                m.push([xr,element[1]]);
            }
        }
        if yu < 40 && yu >= 0 {
            if map[element[0]][yu]== color {
                m.push([element[0],yu]);
            }
        }
        if yd < 40 && yd >= 0 {
            if map[element[0]][yd]== color {
                m.push([element[0],yd]);
            }
        }
        if n > 30 {return 30};
        }
    }
    return n;
}


if "t" in memory{
    memory.t += 1
}
else {
    let x = worker(0).x;
    let y = worker(0).y;
    memory.atacante1stat = 1;
    memory.atacante2stat = 1;
    memory.cuadrante = 0;
    if x < 20 && y < 20 {
        memory.cuadrante = 0;
        memory.atacante1 = [4, "up", "right", "down", "zigzag2"];
        memory.atacante2 = [6, "right", "zigzag2", "right", "down"];
    }
    else if x < 20 && y > 20 {
        memory.cuadrante = 1;
        memory.atacante1 = [6, "right","down", "left", "zigzag7"];
        memory.atacante2 = [3, "down", "zigzag7", "down", "left"];
    }
    else if x > 20 && y < 20 {
        memory.cuadrante = 2;
        memory.atacante1 = [1, "left", "up", "right", "zigzag0"];
        memory.atacante2 = [4, "up", "zigzag0", "up", "right"];
    }
    else if x > 20 && y > 20 {
        memory.cuadrante = 3;
        memory.atacante1 = [3, "down", "left", "up", "zigzag5"];
        memory.atacante2 = [1, "left", "zigzag5", "left", "up"];
    }
    memory.t = 0;
}
fn cuadrante (x, y){
    let cuadrante = 0;
    if x < 20 && y < 20 {
        cuadrante = 0;
    }
    else if x < 20 && y > 20 {
        cuadrante = 1;
    }
    else if x > 20 && y < 20 {
        cuadrante = 2;
    }
    else if x > 20 && y > 20 {
        cuadrante = 3;
    }
    return cuadrante;
}
fn cuadranteZigzag (x, y){
    let zigzag = "";
    if x < 20 && y < 20 {
        zigzag= "zigzag7";
    }
    else if x < 20 && y > 20 {
       zigzag = "zigzag5";
    }
    else if x > 20 && y < 20 {
       zigzag = "zigzag2";
    }
    else if x > 20 && y > 20 {
       zigzag = "zigzag0";
    }
    return zigzag;
}
fn outside_range_xy(x,y){
    return x == 5 || x == 35|| y == 5 || y == 35;
}
fn outside_range_ziga(x,y){
    return x == 0 || x == 39|| y == 0 || y == 39;
}

for w in 0..8 {
    if w == memory.atacante1[0] && memory.t % 20 < 15{
        if outside_range_xy(worker(w).x, worker(w).y){
            memory.atacante1stat += 1;
            if memory.atacante1stat == 5 {memory.atacante1stat = 2}
            let l = (rand() % 2).abs();
            switch cuadrante(worker(w).x, worker(w).y) {
                3 => if l == 0 {worker(w).move_down()} else {worker(w).move_left()},
                2 => if l == 0 {worker(w).move_up()} else {worker(w).move_left()},
                1 => if l == 0 {worker(w).move_down()} else {worker(w).move_right()},
                0 => if l == 0 {worker(w).move_up()} else {worker(w).move_right()},
            }
        }
        else {
            if memory.atacante1stat == 4 {
                memory.atacante1[memory.atacante1stat] = cuadranteZigzag (worker(w).x, worker(w).y);
            }
            switch memory.atacante1[memory.atacante1stat]{
                "up" => worker(w).move_up(),
                "down" => worker(w).move_down(),
                "right" => worker(w).move_right(),
                "left" => worker(w).move_left(),
                "zigzag0" => if memory.t % 2 == 0 {worker(w).move_down()} else {worker(w).move_left()},
                "zigzag2" => if memory.t % 2 == 0 {worker(w).move_left()} else {worker(w).move_up()},
                "zigzag5" => if memory.t % 2 == 0  {worker(w).move_down()} else {worker(w).move_right()},
                "zigzag7" => if memory.t % 2 == 0{worker(w).move_up()} else {worker(w).move_right()},
            }
        }
    }
    else if w == memory.atacante2[0] && memory.t %20 < 15 {
        if outside_range_xy(worker(w).x, worker(w).y){
            memory.atacante2stat += 1;
            if memory.atacante2stat == 5 {memory.atacante2stat = 2}

            let l = (rand() % 2).abs();
            switch cuadrante(worker(w).x, worker(w).y) {
                3 => if l == 0 {worker(w).move_down()} else {worker(w).move_left()},
                2 => if l == 0 {worker(w).move_up()} else {worker(w).move_left()},
                1 => if l == 0 {worker(w).move_down()} else {worker(w).move_right()},
                0 => if l == 0 {worker(w).move_up()} else {worker(w).move_right()},
            }
        }
        else {

            if memory.atacante2stat == 2 {
                memory.atacante2[memory.atacante2stat] = cuadranteZigzag (worker(w).x, worker(w).y);
            }
            switch memory.atacante2[memory.atacante2stat]{
                "up" => worker(w).move_up(),
                "down" => worker(w).move_down(),
                "right" => worker(w).move_right(),
                "left" => worker(w).move_left(),
                "zigzag0" => if memory.t % 2 == 0 {worker(w).move_down()} else {worker(w).move_left()},
                "zigzag2" => if memory.t % 2 == 0 {worker(w).move_left()} else {worker(w).move_up()},
                "zigzag5" => if memory.t % 2 == 0  {worker(w).move_down()} else {worker(w).move_right()},
                "zigzag7" => if memory.t % 2 == 0{worker(w).move_up()} else {worker(w).move_right()},
            }
        }
    }
    else {
        let xPosition = worker(w).x;
        let yPosition = worker(w).y;
        let newPositions = [];
        if xPosition - 1 >= 0 {
            let leftPos = #{
                dir: 0,
                xPos: xPosition - 1,
                yPos: yPosition,
                color: map[xPosition - 1][yPosition],
            };
            newPositions.push(leftPos);
        }
        if xPosition + 1 < 40 {
            let rightPos = #{
                dir: 1,
                xPos: xPosition + 1,
                yPos: yPosition,
                color: map[xPosition + 1][yPosition],
            };
            newPositions.push(rightPos);
        }
        if yPosition + 1 < 40 {
            let upPos = #{
                dir: 2,
                xPos: xPosition,
                yPos: yPosition + 1,
                color: map[xPosition][yPosition + 1],
            };
            newPositions.push(upPos);
        }
        if yPosition - 1 >= 0 {
            let downPos = #{
                dir: 3,
                xPos: xPosition,
                yPos: yPosition - 1,
                color: map[xPosition][yPosition - 1],
            };
            newPositions.push(downPos);
        }
        let movedToEmpty = false;
        for (newPosition, count) in newPositions {
            let newX = newPosition.xPos;
            let newY = newPosition.yPos;
            let dir = newPosition.dir;
            let max = -1;
            if (newPosition.color != memory["myColor"]) {
                if (newPosition.color != Tile::EMPTY) {
                    if !usedPositions.contains([newX,newY]) {
                        let number = ConnectedComponents(newX, newY, map);
                        let number = 0;
                        if max < number {
                            max = number;
                            MoveToDirection(w, dir);
                            if (movedToEmpty) {usedPositions.pop();}
                            usedPositions.push([newX,newY]);
                            movedToEmpty = true;
                        }
                    }
                }
                if !usedPositions.contains([newX,newY]) && !movedToEmpty {
                    MoveToDirection(w, dir);
                    usedPositions.push([newX,newY]);
                    movedToEmpty = true;
                }
            }
        }
        if !movedToEmpty {
            let r = (rand() % 4).abs();
            switch r {
                0 => worker(w).move_up(),
                1 => worker(w).move_down(),
                2 => worker(w).move_right(),
                3 => worker(w).move_left(),
            }
            let l = (rand() % 4).abs();
            let xx = worker(w).x;
            let yy = worker(w).y;
            switch l {
                3 => worker(w).move_down(),
                2 => worker(w).move_left(),
                1 => worker(w).move_right(),
                0 => worker(w).move_up(),
            }
        }
    }
}
