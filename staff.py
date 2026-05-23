"""金箍棒"""

import pygame
import math
from settings import GOLD, STAFF_WIDTH, STAFF_GROW_SPEED, STAFF_ROTATE_SPEED


class Staff:
    def __init__(self):
        self.length = 0
        self.angle = 90  # 竖直向上
        self.base_x = 0
        self.base_y = 0
        self.growing = False
        self.falling = False
        self.landed = False

    def start_grow(self, x, y):
        self.length = 0
        self.angle = 90
        self.base_x = x
        self.base_y = y
        self.growing = True
        self.falling = False
        self.landed = False

    def grow(self, dt):
        if self.growing:
            self.length += STAFF_GROW_SPEED * dt

    def start_fall(self):
        self.growing = False
        self.falling = True

    def fall(self, dt):
        if self.falling and self.angle > 0:
            self.angle -= STAFF_ROTATE_SPEED * dt
            if self.angle <= 0:
                self.angle = 0
                self.falling = False
                self.landed = True

    def get_tip_x(self):
        rad = math.radians(self.angle)
        return self.base_x + self.length * math.cos(rad)

    def get_end_x_when_landed(self):
        return self.base_x + self.length

    def reset(self):
        self.length = 0
        self.angle = 90
        self.growing = False
        self.falling = False
        self.landed = False

    def draw(self, surface, camera_x):
        if self.length <= 0:
            return
        rad = math.radians(self.angle)
        start = (self.base_x - camera_x, self.base_y)
        end_x = self.base_x + self.length * math.cos(rad) - camera_x
        end_y = self.base_y - self.length * math.sin(rad)
        pygame.draw.line(surface, GOLD, start, (end_x, end_y), STAFF_WIDTH)
