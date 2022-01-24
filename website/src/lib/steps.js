export default [

    {
        action: "command",
        command: "curl https://install.envwoman.eu.org | sh",
        output: "Downloading... Installed successfully!",
        // transcription: "Executing the install-script"
        transcription: "COMING SOON!"
    },
    {
        action: "command",
        command: "envwoman login",
        output: "Logging in... Login succeded!",
        transcription: "Login via browser and a secure code"
    },
    {
        action: "command",
        command: "envwoman init -f .env -d \"My first env!\"",
        output: `Create a new project called "cli"? [y/n]: yes`,
        transcription: "Create a new project"
    },
    {
        action: "command",
        command: `echo "HELLO=WORLD" > .env`,
        transcription: "Change your .env",
    },
    {
        action: "command",
        command: "envwoman push",
        output: "Successfully updated envs",
        transcription: "Push the updated env to the server"

    }
	// {
	// 	action: 'command',
	// 	command: "mkdir example",
	// 	transcription: "create directory"
	// },
	// {
	// 	action: 'command',
	// 	command: "cd example",
	// 	transcription: "cd into directory"
	// },
	

	// {
	// 	action: 'command',
	// 	command: 'echo "# Hello World!" > README.md',
	// 	transcription: "create a readme file"
	// },
	// {
	// 	action: 'wait',
	// 	delay: 1000
	// },
	// {
	// 	action: 'command',
	// 	command: 'git init',
	// 	transcription: "initialize git"
	// },
	// {
	// 	action: 'command',
	// 	command: 'ls -a',
	// 	output: 'README.md    .git/'
		
	// },
	// {
	// 	action: 'wait',
	// 	delay: 1000
	// },
	// {
	// 	action: 'command',
	// 	command: 'git add .',
	// 	transcription: "add all files to git"
	// },
	// {
	// 	action: 'command',
	// 	command: "git commit -m 'Initial commit'",
	// 	transcription: "commit files to git"
	// },
	// {
	// 	action: 'command',
	// 	command: "git remote add origin git@github.com:username/example",
	// 	transcription: "add a remote"
	// },
	// {
	// 	action: 'command',
	// 	command: "git push origin master",
	// 	output: 'Done!',
	// 	transcription: "sync changes with remote"
	// },
]
	