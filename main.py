import random

random_number = random.randint(1, 100)
user_input = input("Enter a number between 1 and 100: ")

if user_input.isdigit() and int(user_input) >= 1 and int(user_input) <= 100:
    if int(user_input) == random_number:
        print("You guessed the number!")
    else:
        print("The number was", random_number)
else:
    print("Please enter a number between 1 and 100.")
