import os
import platform
import psutil
import subprocess
from prompt_toolkit import prompt
from prompt_toolkit.completion import Completer, Completion

class CommandCompleter(Completer):
    def __init__(self, available_commands):
        self.available_commands = available_commands

    def get_completions(self, document, complete_event):
        word_before_cursor = document.get_word_before_cursor()
        completions = []

        for cmd in self.available_commands:
            if cmd.startswith(word_before_cursor):
                completions.append(Completion(cmd, -len(word_before_cursor)))

        return completions

class CommandPlusPlus:
    def __init__(self):
        self.username = os.getlogin()
        self.cwd = os.getcwd()

    def get_battery_percentage(self):
        battery = psutil.sensors_battery()
        return battery.percent if battery else "N/A"

    def display_prompt(self):
        battery_percent = self.get_battery_percentage()
        system_info = f"{platform.system()} {platform.release()}"
        prompt_text = f"{self.username}@{system_info} [{battery_percent}%] {self.cwd}\ncommand++> "
        return prompt_text

    def get_available_commands(self):
        # Retrieve available commands dynamically using subprocess on Windows
        try:
            result = subprocess.run(["where", "*.exe"], capture_output=True, text=True, check=True)
            commands = result.stdout.splitlines()
            return sorted(set(commands))
        except subprocess.CalledProcessError:
            return []

    def run(self):
        available_commands = self.get_available_commands()
        command_completer = CommandCompleter(available_commands)

        while True:
            try:
                user_input = prompt(
                    self.display_prompt(),
                    completer=command_completer,
                )
                subprocess.run(user_input, shell=True, check=True)
            except subprocess.CalledProcessError:
                print(f"Command failed: {user_input}")

if __name__ == "__main__":
    cmdpp = CommandPlusPlus()
    cmdpp.run()
