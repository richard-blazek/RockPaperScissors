#include <iostream>
#include <algorithm>
#include <vector>
#include <SDL.h>


class Keyboard
{
private:
	int size;
	const Uint8* keys;
public:
	Keyboard() : keys(SDL_GetKeyboardState(&size)) {}
	bool IsPressed(SDL_Scancode value)const
	{
		return value < size && bool(keys[int(value)]);
	}
};

enum class Cell : unsigned char
{
	Empty,
	Rock,
	Paper,
	Scissors
};

SDL_Color ColorOf(Cell cell)
{
	switch (cell)
	{
	case Cell::Empty:
		return SDL_Color{0, 0, 0, 255};
	case Cell::Rock:
		return SDL_Color{255, 0, 0, 255};
	case Cell::Paper:
		return SDL_Color{0, 255, 0, 255};
	default:
		return SDL_Color{0, 0, 255, 255};
	}
}

Cell Transition(Cell old, std::vector<Cell> neighbors)
{
	switch (old)
	{
	case Cell::Rock:
		return std::find(neighbors.begin(), neighbors.end(), Cell::Paper) != neighbors.end() ? Cell::Paper : Cell::Rock;
	case Cell::Paper:
		return std::find(neighbors.begin(), neighbors.end(), Cell::Scissors) != neighbors.end() ? Cell::Scissors : Cell::Paper;
	case Cell::Scissors:
		return std::find(neighbors.begin(), neighbors.end(), Cell::Rock) != neighbors.end() ? Cell::Rock : Cell::Scissors;
	default:
		return Cell::Empty;
	}
}

constexpr int width = 100, height = 70, zoom = 12;

struct Grid
{
	Cell grid[height][width] = {{Cell::Empty}};
	Cell *operator[](int y)
	{
		return grid[y];
	}
};

int main()
{
	SDL_Init(SDL_INIT_EVERYTHING);
	SDL_Window *window = SDL_CreateWindow("Kámen, nůžky, papír", 50, 50, width * zoom, height * zoom, 0);
	SDL_Renderer *rend = SDL_CreateRenderer(window, -1, 0);
	Keyboard kb;

	Grid grid;
	bool repeat = true;
	while (!SDL_QuitRequested())
	{
		for (int y = 0; y < height; ++y)
		{
			for (int x = 0; x < width; ++x)
			{
				SDL_Rect rect{x * zoom, y * zoom, zoom, zoom};
				SDL_Color color = ColorOf(grid.grid[y][x]);
				SDL_SetRenderDrawColor(rend, color.r, color.g, color.b, 255);
				SDL_RenderFillRect(rend, &rect);
			}
		}
		SDL_RenderPresent(rend);

		int mouse_x = 0, mouse_y = 0;
		if (SDL_GetMouseState(&mouse_x, &mouse_y))
		{
			int x = mouse_x / zoom, y = mouse_y / zoom;
			if (x >= 0 && x < width && y >= 0 && y < height)
			{
				grid[y][x] = kb.IsPressed(SDL_SCANCODE_LCTRL) ? Cell::Rock
						: kb.IsPressed(SDL_SCANCODE_LALT) ? Cell::Paper
						: kb.IsPressed(SDL_SCANCODE_LSHIFT) ? Cell::Scissors
						: Cell::Empty;
			}
		}
		if (kb.IsPressed(SDL_SCANCODE_SPACE))
		{
			Grid new_grid;
			for (int y = 1; y < height - 1; ++y)
			{
				for (int x = 1; x < width - 1; ++x)
				{
					new_grid[y][x] = Transition(grid[y][x], {grid[y-1][x], grid[y+1][x], grid[y][x-1], grid[y][x+1]});
				}
			}
			grid = new_grid;
		}
		SDL_Delay(25);
	}

	SDL_DestroyRenderer(rend);
	SDL_DestroyWindow(window);
	SDL_Quit();
	return 0;
}