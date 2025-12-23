#include <iostream>
#include <fstream>

#define INPUT_FILE "input.txt"

int main() {
    int currentValue = 50;
    int result = 0;

    std::ifstream InFile(INPUT_FILE);
    std::string currentLine;
    while (std::getline(InFile, currentLine)) {
        const int amount = std::stoi(currentLine.substr(1));
        const bool dir = currentLine[0] == 'R';
        currentValue = currentValue + amount * (dir ? 1 : -1);


        if (currentValue >= 100) currentValue %= 100;
        while (currentValue < 0) currentValue += 100;

        if (currentValue == 0) result++;
    }

    std::cout << result << std::endl;
    return 0;
}
