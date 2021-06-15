/*
 * TODO: Add licence here
 */

#include "movement.h"

extern int libant_movement(int num, char action, char** buffer)
{
	for(int i = 0; i < num; ++i)
	{
		if(libant_movement_action(action, buffer) < 0 ) { return -1; }
	}
	return 0;
}

int libant_movement_action(char action, char** buffer)
{
	switch(action)
	{
		case 'w':
		case 'e':
			while(**buffer != ' ' && **buffer)
				(*buffer)++;
			if(**buffer && (action == 'w'))
				(*buffer)++;
			break;
		case 'b':
			while(**buffer != ' ' && **buffer)
				(*buffer)--;
			if(**buffer)
				(*buffer)++;
		case 'l':
			if(*(*buffer+1))
				(*buffer)++;
			break;
		case 'h':
			if(*(*buffer-1))
				(*buffer)--;
			break;
		case 'j':
			{
			int distance_to_start = libant_get_distance_to_line_start(buffer);
			buffer += libant_get_distance_to_line_end(buffer);
			for(int i = 0; i < distance_to_start; ++i)
			{
				if(*(*buffer+1))
					(*buffer)++;
				else
					break;
			}
			break;
			}
		case 'k':
			{
			int distance_to_end = libant_get_distance_to_line_end(buffer);
			buffer -= libant_get_distance_to_line_start(buffer);
			for(int i = 0; i < distance_to_end; ++i)
			{
				if(*(*buffer-1))
					(*buffer)--;
				else
					break;
			}
			break;
			}
		default:
			return -1;
	}
	return 0;
}

int libant_get_distance_to_line_start(char** buffer)
{
	int distance = 0;
	while(*(*buffer-1) && *(*buffer-1) != '\n')
	{
		(*buffer)--;
		distance++;
	}
	return distance;
}

int libant_get_distance_to_line_end(char** buffer)
{
	int distance = 0;
	while(*(*buffer+1) && *(*buffer+1) != '\n')
	{
		(*buffer)++;
		distance++;
	}
	return distance;
}
