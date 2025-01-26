#[cfg(test)]
mod tests {
    use super::super::parse_cubes_command;

    #[test]
    fn test_parse_multiple_cubes() {
        let json_str = r#"{
            "command": "create cubes",
            "parameters": [
                {
                    "x": 2.0,
                    "y": 2.0,
                    "z": 2.0,
                    "width": 1.0,
                    "height": 1.0,
                    "depth": 1.0
                },
                {
                    "x": 3.0,
                    "y": 2.0,
                    "z": 2.0,
                    "width": 1.0,
                    "height": 1.0,
                    "depth": 1.0
                },
                {
                    "x": 4.0,
                    "y": 2.0,
                    "z": 2.0,
                    "width": 1.0,
                    "height": 1.0,
                    "depth": 1.0
                }
            ]
        }"#;

        let commands = parse_cubes_command(json_str);
        
        assert_eq!(commands.len(), 3);
        
        // Check first cube
        assert_eq!(commands[0].get_x(), 2.0);
        assert_eq!(commands[0].get_y(), 2.0);
        assert_eq!(commands[0].get_z(), 2.0);
        assert_eq!(commands[0].get_width(), 1.0);
        assert_eq!(commands[0].get_height(), 1.0);
        assert_eq!(commands[0].get_depth(), 1.0);
        
        // Check second cube
        assert_eq!(commands[1].get_x(), 3.0);
        assert_eq!(commands[1].get_y(), 2.0);
        assert_eq!(commands[1].get_z(), 2.0);
        
        // Check third cube
        assert_eq!(commands[2].get_x(), 4.0);
        assert_eq!(commands[2].get_y(), 2.0);
        assert_eq!(commands[2].get_z(), 2.0);
        
        // Check command string for all cubes
        for cmd in commands {
            assert_eq!(cmd.get_command(), "create cubes");
        }
    }
}
