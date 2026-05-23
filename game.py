"""游戏主逻辑"""

import pygame
from settings import (
    SCREEN_WIDTH, SCREEN_HEIGHT, FPS, SKY_BLUE, WHITE, RED, BLACK,
    STATE_IDLE, STATE_GROWING, STATE_FALLING, STATE_CROSSING,
    STATE_DEAD, STATE_TRANSITIONING, MONKEY_SIZE
)
from monkey import Monkey
from staff import Staff
from platform import PlatformManager


class Game:
    def __init__(self):
        pygame.init()
        self.screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
        pygame.display.set_caption("我的眼睛就是尺")
        self.clock = pygame.time.Clock()
        self.font = pygame.font.Font(None, 36)
        self.big_font = pygame.font.Font(None, 72)
        self.reset()

    def reset(self):
        self.platforms = PlatformManager()
        self.monkey = Monkey(self.platforms.current.right_edge - MONKEY_SIZE - 10)
        self.staff = Staff()
        self.state = STATE_IDLE
        self.score = 0
        self.camera_x = 0
        self.target_camera_x = 0

    def handle_events(self):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                return False
            if event.type == pygame.MOUSEBUTTONDOWN:
                if self.state == STATE_IDLE:
                    self.state = STATE_GROWING
                    staff_x = self.platforms.current.right_edge
                    staff_y = SCREEN_HEIGHT - 200
                    self.staff.start_grow(staff_x, staff_y)
                elif self.state == STATE_DEAD:
                    self.reset()
            if event.type == pygame.MOUSEBUTTONUP:
                if self.state == STATE_GROWING:
                    self.state = STATE_FALLING
                    self.staff.start_fall()
        return True

    def update(self):
        dt = self.clock.get_time() / 1000.0

        if self.state == STATE_GROWING:
            self.staff.grow(dt)

        elif self.state == STATE_FALLING:
            self.staff.fall(dt)
            if self.staff.landed:
                self._check_bridge()

        elif self.state == STATE_CROSSING:
            target_x = self._get_cross_target()
            arrived = self.monkey.walk_to(target_x, dt)
            if arrived:
                staff_end = self.staff.get_end_x_when_landed()
                next_left = self.platforms.next.left_edge
                if staff_end >= next_left:
                    self._on_success()
                else:
                    self.monkey.start_fall()
                    self.state = STATE_DEAD

        elif self.state == STATE_DEAD:
            self.monkey.update_fall(dt)

        elif self.state == STATE_TRANSITIONING:
            self.camera_x += (self.target_camera_x - self.camera_x) * 0.05
            if abs(self.camera_x - self.target_camera_x) < 1:
                self.camera_x = self.target_camera_x
                self.state = STATE_IDLE

    def _check_bridge(self):
        self.state = STATE_CROSSING

    def _get_cross_target(self):
        staff_end = self.staff.get_end_x_when_landed()
        next_left = self.platforms.next.left_edge

        if staff_end >= next_left:
            return self.platforms.next.left_edge + MONKEY_SIZE
        else:
            return staff_end

    def _on_success(self):
        self.score += 1
        self.staff.reset()
        self.platforms.advance()
        self.monkey.set_position(
            self.platforms.current.right_edge - MONKEY_SIZE - 10
        )
        self.target_camera_x = self.platforms.current.x - 50
        self.state = STATE_TRANSITIONING

    def draw(self):
        self.screen.fill(SKY_BLUE)
        self.platforms.draw(self.screen, self.camera_x)
        self.staff.draw(self.screen, self.camera_x)
        self.monkey.draw(self.screen, self.camera_x)

        score_text = self.font.render(f"Score: {self.score}", True, BLACK)
        self.screen.blit(score_text, (10, 10))

        if self.state == STATE_IDLE:
            hint = self.font.render("Hold mouse to extend staff", True, BLACK)
            self.screen.blit(hint, (SCREEN_WIDTH // 2 - 130, 50))

        if self.state == STATE_DEAD and self.monkey.dead:
            over_text = self.big_font.render("GAME OVER", True, RED)
            rect = over_text.get_rect(center=(SCREEN_WIDTH // 2, SCREEN_HEIGHT // 3))
            self.screen.blit(over_text, rect)
            retry = self.font.render("Click to retry", True, BLACK)
            self.screen.blit(retry, (SCREEN_WIDTH // 2 - 70, SCREEN_HEIGHT // 3 + 50))

        pygame.display.flip()

    def run(self):
        running = True
        while running:
            self.clock.tick(FPS)
            running = self.handle_events()
            self.update()
            self.draw()
        pygame.quit()
