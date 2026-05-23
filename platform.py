"""平台（悬崖）管理"""

import random
import pygame
from settings import (
    PLATFORM_HEIGHT, PLATFORM_MIN_WIDTH, PLATFORM_MAX_WIDTH,
    GAP_MIN, GAP_MAX, SCREEN_WIDTH, SCREEN_HEIGHT,
    BROWN, DARK_BROWN, GREEN
)


class Platform:
    def __init__(self, x, width):
        self.x = x
        self.width = width
        self.y = SCREEN_HEIGHT - PLATFORM_HEIGHT

    @property
    def right_edge(self):
        return self.x + self.width

    @property
    def left_edge(self):
        return self.x

    def draw(self, surface):
        rect = pygame.Rect(self.x, self.y, self.width, PLATFORM_HEIGHT)
        pygame.draw.rect(surface, BROWN, rect)
        pygame.draw.rect(surface, DARK_BROWN, rect, 3)
        grass_rect = pygame.Rect(self.x, self.y, self.width, 10)
        pygame.draw.rect(surface, GREEN, grass_rect)


class PlatformManager:
    def __init__(self):
        self.current = Platform(0, 150)
        self.next = self._generate_next()
        self.offset = 0

    def _generate_next(self):
        gap = random.randint(GAP_MIN, GAP_MAX)
        width = random.randint(PLATFORM_MIN_WIDTH, PLATFORM_MAX_WIDTH)
        x = self.current.right_edge + gap
        return Platform(x, width)

    def advance(self):
        self.current = self.next
        self.next = self._generate_next()

    def get_gap(self):
        return self.next.left_edge - self.current.right_edge

    def draw(self, surface, camera_x):
        for plat in [self.current, self.next]:
            shifted = Platform(plat.x - camera_x, plat.width)
            shifted.draw(surface)
