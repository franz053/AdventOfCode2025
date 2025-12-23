#include <iostream>
#include <fstream>

#define INPUT_FILE "input.txt"

int main() {
    int currentValue = 50;
    int result = 0;

    std::ifstream InFile(INPUT_FILE);
    std::string currentLine;
    while (std::getline(InFile, currentLine)) {
        const bool dir = currentLine[0] == 'R';
        const int amount = std::stoi(currentLine.substr(1)) * (dir ? 1 : -1);
        currentValue = currentValue + amount;

        if (currentValue == 0) {
            result++;
        } else if (currentValue >= 100) {
            result += currentValue / 100;
            currentValue %= 100;
        } else if (currentValue < 0) {
            if (currentValue - amount) result++;
            while (currentValue <= -100) {
                result++;
                currentValue += 100;
            }
            if (currentValue < 0)currentValue += 100;
        }
    }
    std::cout << result << std::endl;
    return 0;
}
