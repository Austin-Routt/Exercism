import math
import random

class Character:
    def __init__(self):
        self.strength = self.ability()
        self.dexterity = self.ability()
        self.constitution = self.ability()
        self.intelligence = self.ability()
        self.wisdom = self.ability()
        self.charisma = self.ability()
        self.hitpoints = 10 + modifier(self.constitution)

    @staticmethod
    def ability():
        rolls = [random.randint(1, 6) for i in range(4)]
        rolls = sorted(rolls)
        rolls.pop(0)
        return sum(rolls)


def modifier(x):
    "subtract 10, divide by 2 and round down."
    return math.floor((x - 10) / 2)
