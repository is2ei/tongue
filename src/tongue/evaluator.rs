// Copyright 2017 Issei Horie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::process::Command;

use tongue::builtin;
use tongue::config::Config;
use tongue::node::Node;

pub fn exec(command: String, options: Vec<String>, mut config: &mut Config) {

}

pub fn eval(tree: Node, mut config: &mut Config) {
    if tree.v.is_empty() {
        return;
    }

    let mut current_node: Node = tree;
    let mut options: Vec<String> = Vec::new();
    loop {
        if current_node.child.is_empty() {
            match current_node.next {
                Some(n) => {
                    break;
                },
                None => {
                    exec("cd".to_string(), options
                    break;
                }
            }
        } else {
            
        }
        break;
    }

    
//    if tree.v == "." {
//        builtin::dot();
//    } else if tree.v == "alias" {
//    } else if tree.v == "break" {
//    } else if tree.v == "cd" {
//        let options: Vec<String> = Vec::new();
//        builtin::cd(options);
//        
//    } else if tree.v == "continue" {
//    } else if tree.v == "exec" {
//    } else if tree.v == "exit" {
//    } else if tree.v == "export" {
//    } else if tree.v == "return" {
//    } else {
//        if options.is_empty() {
//            Command::new(command)
//                .status()
//                .expect("command failed to start");
//        } else {
//            Command::new(command)
//               .args(options)
//               .status()
//               .expect("command failed to start");
//        }
//    }      
}

