

// universal constants

use std::io;
use std::fs;
fn main() {
    println!("Hello, world!");
    
    // let mut category = String::new();
    // println!("Please enter a category: 'kinematics' 'dynamics' 'energetics'");
    // io::stdin().read_line(&mut category).expect("Could not read line");
    // not needed
    let mut inputs = String::new();
    println!("Please enter what you wish to find as well as given numbers in m/s/kg form: 
    \n eg:  'a vi=0 t=10 d=20'");
    io::stdin().read_line(&mut inputs).expect("Could not read line");
    let equations_file = fs::read_to_string("skiatesthing/src/equations.txt").unwrap();
    let val_to_find = inputs.split(';').next().unwrap();
    let possible: Vec<String> = get_possible(equations_file, val_to_find);
    let mut v_ins = inputs.split_ascii_whitespace();
    v_ins.next();
    // eg {vi=0,d=20,t=24}
    let mut equation = String::new();
    for eq in possible{
        let mut eq: Vec<&str> = eq.split(';').collect();
        eq.reverse();
        eq.pop();
        let req = eq.clone().pop().unwrap().split_ascii_whitespace();
        let temp_vec:Vec<&str> = v_ins.clone().collect();
        let temp_req:Vec<&str> = req.collect();
        // eg. {d,a,t}
        let any_works: bool = check_equation(&temp_vec, &temp_req);
        
        // exits loop when equation is found to be more efficient
        if any_works {
            equation.push_str(eq.pop().clone().unwrap());
            break;
        }
        
    }

    // stops program if equation is not found
    if equation.is_empty(){
        panic!("could not find equation!");
    }
    
    // evaluate expression
}

fn check_equation(line: &Vec<&str>, input: &Vec<&str>   )-> bool{
    // returns true if given values match needed values
    
    //let mut var_vec: Vec<&str> = line.clone().pop().unwrap().split_ascii_whitespace().collect();
    let mut t2 = input.clone();
    t2.sort();
    t2.reverse();
    
    print!("{:#?}",line);
    // todo: sort vecs so it can work in any order or alt check through given and see if var exist then pop from given
    for var in line{
        let t3 = var.split('=').next().unwrap();
        let cur = t2.pop().unwrap().split('=').next().unwrap();
        print!("{:#?}",cur);
        println!("{:#?}",t3);
        if !t3.eq(cur) {
            return false;
        }
    }
    return true;
    
}

fn get_possible(file: String, pattern: &str) -> Vec<String>{
    let temp = file.lines();
    let mut possible = Vec::new();
    for line in temp{
        if line.starts_with(pattern) {
            possible.push(String::from(line));
        }
    }
    return possible;
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "functional"]
    fn test_poss(){ //Functional
        let test_in: Vec<&str> = vec!["vi=0","t=10","d=20"];
        // Fixed: functional
        let files = fs::read_to_string("src/equations1.txt").unwrap();
        let mut poss = get_possible(files, "a");
        let expected_vec: Vec<&str> = vec!["a;  vi d t       ;    (d - 'vi' * 't') / 0.5 * 't' * 't'","a;  vf vi t      ;    ('vf' - 'vi') / 't'","a;  vf vi d      ;    (('vf' * 'vf') - ('vi' * 'vi')) / (2 * d)","a;  fnet m       ;    'fnet' / 'm'"];
        assert_eq!(expected_vec, poss);
        
    }
    #[test]
    
    fn test_eq(){
        let test_in: Vec<&str> = vec!["vi=0","t=10","d=20"];
        let test2_in: Vec<&str> = vec!["vf=0","t=10","d=20"];
        let t3_in: Vec<&str> = vec!["fa=29","W=34"];
        let mut eq: Vec<&str> = "a; d  t  vi ;    (d - 'vi' * 't') / 0.5 * 't' * 't'".split(';').collect();
        let mut e2: Vec<&str> =  "d;                       W  fa ;    'W' / 'fa'".split(';').collect();
        e2.pop();
        eq.pop();
        let l2:Vec<&str> = e2.clone().pop().unwrap().split_ascii_whitespace().collect();
        let req = eq.clone().pop().unwrap().split_ascii_whitespace();
        let temp_req:Vec<&str> = req.collect();
        print!("{:#?}",test2_in);
        assert_eq!(check_equation(&temp_req, &test_in), true);
        assert_eq!(check_equation(&temp_req, &test2_in), false);
        assert_eq!(check_equation(&l2, &t3_in),true);
    }
}

/*
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.*;

public class Main {

    Vector<Vector<Character>> upright;
    Vector<Vector<Character>> diagonal_one;
    Vector<Vector<Character>> diagonal_two;
    String keyword;
    int rows;
    int columns;
    public static void main(String[] args) {
        Main main = new Main();
        main.upright = new Vector<>();
        InputStreamReader streamReader = new InputStreamReader(System.in);
        BufferedReader reader = new BufferedReader(streamReader);
        try {
            int counter = 0;
            main.keyword = reader.readLine();
            main.rows = Integer.parseInt(reader.readLine());
            main.columns = Integer.parseInt(reader.readLine());
            Vector starts = new Vector<>();
            for (int i = 0; i < main.rows; i++) {
                String row = reader.readLine();
                String[] tmp3 = row.split(" ");
                Vector tmp2 = new Vector<>();
                for (int j = 0; j < main.columns; j++) {
                    tmp2.add(tmp3[j].charAt(0));
                    if (tmp3[j].charAt(0) == main.keyword.charAt(0)){
                        starts.add(new Pair(i,j));
                    }
                }
                main.upright.add(tmp2);
            }

            for (Object p :
                    starts) {
                Pair p2 = (Pair) p;
                counter += main.word_exists((byte) 1,false,p2,Dir.NONE);
            }
            System.out.println(counter);

        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }





    public int word_exists(byte letter,boolean bent, Pair point, Dir last_dir){
        char let_to_find = keyword.charAt(letter);
        boolean last_letter = keyword.length() - (letter + 1) == 0;
        if (bent) {
            switch (last_dir){
                case UP:
                    if (point.x == 0) {
                        return 0;
                    } else {
                        if (point.Char_at_point(point.x - 1, point.y,this) == let_to_find) {
                            if (last_letter) {
                                return 1;
                            } else {
                                return word_exists((byte) (letter + 1), true, new Pair(point.x - 1, point.y),last_dir);
                            }
                        } else return 0;
                    }
                case DOWN:
                        if (point.x == rows - 1) {
                        return 0;
                    } else {
                        if (point.Char_at_point(point.x + 1, point.y,this) == let_to_find) {
                            if (last_letter) {
                                return 1;
                            } else {
                                return word_exists((byte) (letter + 1), true, new Pair(point.x + 1, point.y),last_dir);
                            }
                        } else return 0;
                    }
                case LEFT:
                    if (point.y == 0) {
                        return 0;
                    } else {
                        if (point.Char_at_point(point.x, point.y - 1,this) == let_to_find) {
                            if (last_letter) {
                                return 1;
                            } else {
                                return word_exists((byte) (letter + 1), true, new Pair(point.x, point.y - 1),last_dir);
                            }
                        } else return 0;
                    }
                case RIGHT:
                    if (point.y == columns - 1) {
                        return 0;
                    } else {
                        if (point.Char_at_point(point.x, point.y + 1,this) == let_to_find) {
                            if (last_letter) {
                                return 1;
                            } else {
                                return word_exists((byte) (letter + 1), true, new Pair(point.x , point.y + 1),last_dir);
                            }
                        } else return 0;
                    }
            } //check same direction
        } else {
            char up =point.x == 0? ' ' : upright.get(point.x-1).get(point.y);
            char left =point.y == 0 ? ' ' : upright.get(point.x).get(point.y-1);
            char down = point.x == rows - 1 ? ' ' : upright.get(point.x+1).get(point.y);
            char right =point.y == columns - 1 ? ' ' : upright.get(point.x).get(point.y+1);
            Vector<Character> tmp = new Vector<>();
            tmp.add(up); tmp.add(left); tmp.add(down); tmp.add(right);
            if (last_dir != Dir.NONE) {
                tmp.set(Dir.int_representation(last_dir),' '); // turn prev location as blank
            }
            int counter = 0;
            for (int i = 0; i < 4; i++) {
                char cur = tmp.get(i);
                if (cur == let_to_find) {
                    if (last_letter) {
                        return 1;
                    }else {
                        // if not opposite
                        // and not last letter
                        Pair next_p = null;
                        Dir next_d = null;
                        switch (i){
                            case 0:
                                next_p = new Pair(point.x-1,point.y);
                                next_d = Dir.UP;
                                break;
                            case 1:
                                next_p = new Pair(point.x,point.y-1);
                                next_d = Dir.LEFT;
                                break;
                            case 2:
                                next_p = new Pair(point.x+1,point.y);
                                next_d = Dir.DOWN;
                                break;
                            case 3:
                                next_p = new Pair(point.x,point.y+1);
                                next_d = Dir.RIGHT;
                                break;

                        }
                        if (i != Dir.int_representation(Dir.opposite(last_dir)) && last_dir != Dir.NONE){
                            counter += word_exists((byte) (letter+1),true,next_p,next_d);
                        }
                        else {
                            counter += word_exists((byte) (letter+1),false,next_p,last_dir);
                        }
                        // if opposite and not last letter
                    }
                }
            }
            return counter;
        }
        return 0;
    }



    static class Pair {
        public int x;
        public int y;
        public Pair(int x, int y){
            this.x = x;
            this.y = y;
        }
        public char Char_at_point(Main main){
            return main.upright.get(x).get(y);
        }
        public char Char_at_point(int r,int c, Main main){
            return main.upright.get(r).get(c);
        }
    }
    enum Dir{
        UP,
        DOWN,
        LEFT,
        RIGHT,
        NONE;
        public static Dir opposite(Dir dir){
            switch (dir){
                case UP:
                    return DOWN;

                case DOWN:
                    return UP;

                case LEFT:
                    return RIGHT;

                case RIGHT:
                    return LEFT;

            }
            return NONE;
        }
        public static int int_representation(Dir dir){
            switch (dir){
                case UP:
                    return 0;
                case DOWN:
                    return 2;
                case LEFT:
                    return 1;
                case RIGHT:
                    return 3;
                default:
                    return -1;
            }
        }

    }
}
 */