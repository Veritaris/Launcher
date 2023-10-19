package pro.gravit.launchserver.cli;


import pro.gravit.utils.command.Command;
import pro.gravit.utils.command.CommandHandler;

import java.util.Arrays;


public class CliExecutor {

    public static void printHelp(CommandHandler handler) {
        StringBuilder builder = new StringBuilder();
        CliCommand command;
        Command lsCommand;
        builder.append("Available commands for CLI:\n");
        for (int i = 0; i < CliCommand.values().length; i++) {
            command = CliCommand.values()[i];
            lsCommand = handler.findCommand(command.command);
            builder.append(String.format(
                " * %s %s - %s\n",
                command.command,
                lsCommand.getArgsDescription() != null ? lsCommand.getArgsDescription() : "[nothing]",
                lsCommand.getUsageDescription()
            ));
        }
        System.out.println(builder);
    }

    public static void execute(CommandHandler handler, String[] args) {
        if (args[0].equalsIgnoreCase("help")) {
            printHelp(handler);
            return;
        }
        Command command = handler.findCommand(args[0]);
        if (command == null) {
            System.out.printf("Requested command \"%s\" not found, shutting down silently\n", args[0]);
            return;
        }
        CliCommand cliCommand = CliCommand.valueOf(args[0].toUpperCase());
        if ((args.length - 1) < cliCommand.requiredArgc) {
            System.out.printf(
                "Not enough args for command \"%s\": %d expected but %d provided\n",
                args[0], cliCommand.requiredArgc, args.length - 1
            );
            return;
        }
        try {
            System.out.printf("args: %s\n", Arrays.toString(Arrays.copyOfRange(args, 1, args.length)));
            command.invoke(Arrays.copyOfRange(args, 1, args.length));
        } catch (Exception e) {
            System.out.printf(
                "An error occurred during executing command \"%s\", args: %s\n",
                args[0], Arrays.toString(args)
            );
            e.printStackTrace();
        }
    }

    public enum CliCommand {
        HELP("help", 0),
        BUILD("build", 0),
        SIGNJAR("signjar", 1),
        SIGNDIR("signdir", 1),
        OSSLSIGNEXE("osslsignexe", 2),
        GENERATECERTIFICATE("generatecertificate", 0);
        final String command;
        final int requiredArgc;

        CliCommand(String command, int requiredArgc) {
            this.command = command;
            this.requiredArgc = requiredArgc;
        }
    }
}
