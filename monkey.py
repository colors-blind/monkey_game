"""猴子角色"""

import pygame
from settings import (
    MONKEY_SIZE, MONKEY_SPEED, SCREEN_HEIGHT, PLATFORM_HEIGHT,
    ORANGE, RED, WHITE, BLACK
)


class Monkey:
    def __init__(self, x):
        self.x = x
        self.y = SCREEN_HEIGHT - PLATFORM_HEIGHT - MONKEY_SIZE
        self.speed = MONKEY_SPEED
        self.falling = False
        self.fall_speed = 0
        self.dead = False

    def walk_to(self, target_x, dt):
        if self.x < target_x:
            self.x += self.speed * dt
            if self.x >= target_x:
                self.x = target_x
                return True
        return False

    def start_fall(self):
        self.falling = True
        self.fall_speed = 0

    def update_fall(self, dt):
        if self.falling:
            self.fall_speed += 800 * dt
            self.y += self.fall_speed * dt
            if self.y > SCREEN_HEIGHT + 100:
                self.dead = True

    def set_position(self, x):
        self.x = x
        self.y = SCREEN_HEIGHT - PLATFORM_HEIGHT - MONKEY_SIZE

    def draw(self, surface, camera_x):
        draw_x = self.x - camera_x
        draw_y = self.y

        # 身体
        body_rect = pygame.Rect(draw_x, draw_y + 15, MONKEY_SIZE, MONKEY_SIZE - 15)
        pygame.draw.ellipse(surface, ORANGE, body_rect)

        # 头
        head_rect = pygame.Rect(draw_x + 8, draw_y, 24, 22)
        pygame.draw.ellipse(surface, ORANGE, head_rect)

        # 脸
        face_rect = pygame.Rect(draw_x + 12, draw_y + 6, 16, 14)
        pygame.draw.ellipse(surface, (255, 220, 180), face_rect)

        # 眼睛
        pygame.draw.circle(surface, WHITE, (int(draw_x + 16), int(draw_y + 11)), 3)
        pygame.draw.circle(surface, WHITE, (int(draw_x + 24), int(draw_y + 11)), 3)
        pygame.draw.circle(surface, BLACK, (int(draw_x + 16), int(draw_y + 11)), 1)
        pygame.draw.circle(surface, BLACK, (int(draw_x + 24), int(draw_y + 11)), 1)
