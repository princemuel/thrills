"""Birthday Paradox Simulation, by Al Sweigart al@inventwithpython.com
Explore the surprising probabilities of the "Birthday Paradox".
More info at https://en.wikipedia.org/wiki/Birthday_problem
View this code at https://nostarch.com/big-book-small-python-projects
Tags: short, math, simulation"""

import datetime
import random
from typing import List


def main():
    # Display the intro:
    print("""Birthday Paradox, by Al Sweigart al@inventwithpython.com

    The Birthday Paradox shows us that in a group of N people,
    the odds that two of them have matching birthdays is surprisingly large.
    This program does a Monte Carlo simulation (that is, repeated random simulations) to explore this concept.

    (It's not actually a paradox, it's just a surprising result.)""")

    # Set up a tuple of month names in order:
    MONTHS = (
        "Jan",
        "Feb",
        "Mar",
        "Apr",
        "May",
        "Jun",
        "Jul",
        "Aug",
        "Sep",
        "Oct",
        "Nov",
        "Dec",
    )
    MAX_BIRTHDAYS = 100
    NUM_SIMULATIONS = 100_000

    while True:  # Keep asking until the user enters a valid amount.
        print(f"How many birthdays shall I generate? (Max {MAX_BIRTHDAYS})")
        response = input("> ")
        if response.isdecimal() and (0 < int(response) <= MAX_BIRTHDAYS):
            numBDays = int(response)
            break  # User has entered a valid amount.

    print()

    # Generate and display the birthdays:
    print(f"Here are {numBDays} birthdays:")
    birthdays = getBirthdays(numBDays)

    for i, birthday in enumerate(birthdays):
        if i != 0:
            # Display a comma for each birthday after the first birthday.
            print(", ", end="")
        monthName = MONTHS[birthday.month - 1]
        dateText = f"{monthName} {birthday.day}"
        print(dateText, end="")

    print()
    print()

    # Determine if there are two birthdays that match.
    match = getMatch(birthdays)

    # Display the results:
    print("In this simulation, ", end="")
    if match is not None:
        monthName = MONTHS[match.month - 1]
        dateText = f"{monthName}{match.day}"
        print("multiple people have a birthday on", dateText)
    else:
        print("there are no matching birthdays.")
    print()

    # Run through default number of simulations:
    print(f"Generating {numBDays} random birthdays {NUM_SIMULATIONS} times...")
    input("Press Enter to begin...")

    print(f"Let's run another {NUM_SIMULATIONS} simulations.")
    simMatch = 0  # How many simulations had matching birthdays in them.
    for i in range(NUM_SIMULATIONS):
        # Report on the progress every len_sim/10 simulations:
        if i % (NUM_SIMULATIONS / 100) == 0:
            print(f"{i} simulations run...")

        birthdays = getBirthdays(numBDays)
        if getMatch(birthdays) is not None:
            simMatch = simMatch + 1
    print(f"{NUM_SIMULATIONS} simulations run.")

    # Display simulation results:
    probability = round(simMatch / NUM_SIMULATIONS * MAX_BIRTHDAYS, 2)
    print(f"""Out of {NUM_SIMULATIONS} simulations of {numBDays} people,
there was a matching birthday in that group {simMatch} times.
This means that {numBDays} people have a {probability}% chance
of having a matching birthday in their group.
That's probably more than you would think!
""")


def getBirthdays(numberOfBirthdays: int) -> List[datetime.date]:
    """Returns a list of number random date objects for birthdays."""
    birthdays = []
    for _ in range(numberOfBirthdays):
        # The year is unimportant for our simulation, as long as all birthdays have the same year.
        startOfYear = datetime.date(2001, 1, 1)

        # Get a random day into the year:
        randomNumberOfDays = datetime.timedelta(random.randint(0, 364))
        birthday = startOfYear + randomNumberOfDays
        birthdays.append(birthday)
    return birthdays


def getMatch(birthdays: List[datetime.date]):
    """Returns the date object of a birthday that occurs more than once in the birthdays list."""
    if len(birthdays) == len(set(birthdays)):
        return None  # All birthdays are unique, so return None.

    # Compare each birthday to every other birthday:
    for a, birthdayA in enumerate(birthdays):
        for _, birthdayB in enumerate(birthdays[a + 1 :]):
            if birthdayA == birthdayB:
                return birthdayA  # Return the matching birthday.


# If the program is run (instead of imported), run the game:
if __name__ == "__main__":
    main()
