#include <iostream>
#include <vector>
#include "objsdl/objsdl.h"
#include "mylibraries/func.h"

using namespace std;

SDL::Color NewColor(SDL::Color old, SDL::Color up, SDL::Color down, SDL::Color left, SDL::Color right)
{
	if(old.r==255)
	{
		return SDL::Color(255-func::Max(up.g, down.g, right.g, left.g), func::Max(up.g, down.g, right.g, left.g), 0);
	}
	else if(old.g==255)
	{
		return SDL::Color(0, 255-func::Max(up.b, down.b, right.b, left.b), func::Max(up.b, down.b, right.b, left.b));
	}
	else if(old.b==255)
	{
		return SDL::Color(func::Max(up.r, down.r, right.r, left.r), 0, 255-func::Max(up.r, down.r, right.r, left.r));
	}
	return SDL::Color(0,0,0);
}

int main(int argc, const char* argv[])try
{
	SDL::Window window("Kámen, nůžky, papír", SDL::Rect(70,70, 900, 600));
	SDL::Renderer rend(window);
	SDL::KeyboardState kb;
	SDL::Surface field(SDL::Point(90,60), {SDL::Color(0,0,0), SDL::Color(255,0,0), SDL::Color(0,255,0), SDL::Color(0,0,255)}, 8);
	bool repeat=true;
	uint8 paintbrush_size=1;
	while(repeat)
	{
		rend.Draw(field, field.Size(), rend.Size());
		rend.Show();
		if(SDL::Cursor::IsPressed())
		{
			field.Draw(SDL::Cursor::Position()/10,
					kb.IsPressed(SDL::Scancode::R)?SDL::Color(255,0,0):
					kb.IsPressed(SDL::Scancode::G)?SDL::Color(0,255,0):
					kb.IsPressed(SDL::Scancode::B)?SDL::Color(0,0,255):
					SDL::Color(0,0,0));
		}
		for(auto& event:SDL::events::Handler())
		{
			if(event.Type()==SDL::events::Type::Quit)
			{
				repeat=false;
			}
			else if(event.Type()==SDL::events::Type::MouseWheel)
			{
				paintbrush_size+=event.MouseWheel().Move.y;
			}
		}
		if(kb.IsPressed(SDL::Scancode::X))
		{
            SDL::Surface new_field(field.Size(), {SDL::Color(0,0,0), SDL::Color(255,0,0), SDL::Color(0,255,0), SDL::Color(0,0,255)}, 8);
            for(int x=1; x<field.Size().x-1; ++x)
			{
				for(int y=1; y<field.Size().y-1; ++y)
				{
					new_field.Draw(SDL::Point(x,y), NewColor(field[SDL::Point(x,y)], field[SDL::Point(x,y-1)], field[SDL::Point(x,y+1)], field[SDL::Point(x-1,y)], field[SDL::Point(x+1,y)]));
				}
			}
			field=func::Move(new_field);
		}
		SDL::Wait(20);
	}
	return 0;
}
catch(exception& exc)
{
	SDL::MessageBox::Show("Chyba", "Chybová hláška: '"s+exc.what()+"'");
}