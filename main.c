#include <SDL2/SDL.h>

#define WIDTH 100
#define HEIGHT 70
#define ZOOM 12

int is_pressed(SDL_Scancode value)
{
	return SDL_GetKeyboardState(0)[value];
}

typedef enum cell
{
	Empty,
	Rock,
	Paper,
	Scissors
} cell;

SDL_Color color_of(cell c)
{
	SDL_Color color = {c == Rock ? 255 : 0, c == Paper ? 255 : 0, c == Scissors ? 255 : 0, 255};
	return color;
}

cell new_generation(cell old, cell right, cell down, cell left, cell up)
{
	switch (old)
	{
	case Rock:
		return right == Paper || down == Paper || left == Paper || up == Paper ? Paper : Rock;
	case Paper:
		return right == Scissors || down == Scissors || left == Scissors || up == Scissors ? Scissors : Paper;
	case Scissors:
		return right == Rock || down == Rock || left == Rock || up == Rock ? Rock : Scissors;
	default:
		return Empty;
	}
}

typedef struct playground
{
	cell data[HEIGHT][WIDTH];
} playground;

int main()
{
	SDL_Init(SDL_INIT_EVERYTHING);
	SDL_Window *window = SDL_CreateWindow("Kámen, nůžky, papír", 50, 50, WIDTH * ZOOM, HEIGHT * ZOOM, 0);
	SDL_Renderer *rend = SDL_CreateRenderer(window, -1, 0);

	playground grid = {{{Empty}}};
	while (!SDL_QuitRequested())
	{
		for (int y = 0; y < HEIGHT; ++y)
		{
			for (int x = 0; x < WIDTH; ++x)
			{
				SDL_Rect rect = {x * ZOOM, y * ZOOM, ZOOM, ZOOM};
				SDL_Color color = color_of(grid.data[y][x]);
				SDL_SetRenderDrawColor(rend, color.r, color.g, color.b, 255);
				SDL_RenderFillRect(rend, &rect);
			}
		}
		SDL_RenderPresent(rend);

		int mouse_x = 0, mouse_y = 0;
		if (SDL_GetMouseState(&mouse_x, &mouse_y))
		{
			int x = mouse_x / ZOOM, y = mouse_y / ZOOM;
			if (x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT)
			{
				grid.data[y][x] = is_pressed(SDL_SCANCODE_LCTRL) ? Rock
								: is_pressed(SDL_SCANCODE_LALT) ? Paper
								: is_pressed(SDL_SCANCODE_LSHIFT) ? Scissors
								: Empty;
			}
		}
		if (is_pressed(SDL_SCANCODE_SPACE))
		{
			playground new;
			for (int y = 1; y < HEIGHT - 1; ++y)
			{
				for (int x = 1; x < WIDTH - 1; ++x)
				{
					new.data[y][x] = new_generation(grid.data[y][x], grid.data[y][x+1], grid.data[y+1][x], grid.data[y][x-1], grid.data[y-1][x]);
				}
			}
			grid = new;
		}
		SDL_Delay(25);
	}

	SDL_DestroyRenderer(rend);
	SDL_DestroyWindow(window);
	SDL_Quit();
	return 0;
}