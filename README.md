# Robot Rename Command Generator
This is a program to help configure Studica robots. Built in a classroom environment for VMXpi robots and RealVNC Viewer, this program validates and generates a command to configure the robot given a name, number and password (if enabled). By default, password configuration is off, and the set password is `password`. The generated command can be pasted into a terminal window inside RealVNC Viewer.

# Usage
When the program starts up, all available fields are empty. The command will not generate until all fields are filled in and valid. Once you fill in the robot's name, number, and password if enabled, then the command will show. Click `Copy` to copy the command, then you can paste it into your RealVNC Viewer window. It is *highly* recommended to **not** modify the command once pasted into RealVNC viewer.

# Restrictions
The restrictions to each field are as follows:

- No field can contain whitespace
- No field can be empty
- Name cannot start with a number
- Number must be a number and a number only
- Number must be between 0-9999 inclusive

# Password Configuration
`robot-rename.exe` file does not allow password configuration by default. If you want to allow password configuration, you can:
- Run the `robot-rename-fpass.exe` file (compiled with password field force enabled)
- Run the `robot-rename-password.bat` file in the same directory as `robot-rename.exe`, or
- Run `robot-rename.exe` with the environment variable `ROBOT_RENAME_CFG_PASSWORD` defined to any value (which is what the batch file does).
