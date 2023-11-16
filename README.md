# Robot Rename Command Generator
This is a program to help configure Studica robots. Built in a classroom environment for VMXpi robots and RealVNC Viewer, this program validates and generates a command to configure the robot given a name, number and password (if enabled). By default, password configuration is off, and the default password is `password`. The generated command can be pasted into a terminal window inside RealVNC Viewer.

There are restrictions to each field:

- No field can contain whitespace
- No field can be empty
- Name cannot start with a number
- Number must *be a number*
- Number must be between 0-9999 inclusive

# Password Configuration
This is a command generator, so you can change the password after it has been generated.
If you want to allow password configuration, there are three ways to enable it within the app.

Prebuilt forced: The first way is by downloading and running the `robot-rename-fpass.exe` file. This is built to always show the password.

The other ways involve `robot-rename.exe`:

Batch file: You can run the `robot-rename-password.bat` file. This will configure environment variables for the program.

Terminal: If you're running the program in a terminal, run `set ROBOT_RENAME_CFG_PASSWORD=1`, then run the program in the same terminal. This is what the batch file does automatically.
