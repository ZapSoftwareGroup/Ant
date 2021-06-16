/*
 * TODO: Add licence here
 */

#include "movement.h"

extern int libant_movement(int num, char action, struct libant_buffer* buf)
{
	for(int i = 0; i < num; ++i)
	{
		if(libant_movement_action(action, buf) < 0 ) { return -1; }
	}
	return 0;
}

int libant_movement_action(char action, struct libant_buffer* buf)
{
	switch(action)
	{
		case 'w':
		case 'e':
			while(*buf->ptr != ' ' && *buf->ptr)
				(buf->ptr)++;
			if(*buf->ptr && (action == 'w'))
				(buf->ptr)++;
			break;
		case 'b':
			while(*buf->ptr != ' ' && *buf->ptr)
				buf->ptr--;
			if(*buf->ptr)
				buf->ptr++;
		case 'l':
			if(*(buf->ptr+1))
				buf->ptr++;
			break;
		case 'h':
			if(*(buf->ptr-1))
				buf->ptr--;
			break;
		case 'j':
			{
			int distance_to_start = libant_get_distance_to_line_start(buf);
			buf->ptr += libant_get_distance_to_line_end(buf);
			for(int i = 0; i < distance_to_start; ++i)
			{
				if(*(buf->ptr+1))
					(buf->ptr)++;
				else
					break;
			}
			break;
			}
		case 'k':
			{
			int distance_to_end = libant_get_distance_to_line_end(buf);
			buf->ptr -= libant_get_distance_to_line_start(buf);
			for(int i = 0; i < distance_to_end; ++i)
			{
				if(*(buf->ptr-1))
					buf->ptr--;
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

int libant_get_distance_to_line_start(struct libant_buffer* buf)
{
	int distance = 0;
	while(*(buf->ptr-1) && *(buf->ptr-1) != '\n')
	{
		buf->ptr--;
		distance++;
	}
	return distance;
}

int libant_get_distance_to_line_end(struct libant_buffer* buf)
{
	int distance = 0;
	while(*(buf->ptr+1) && *(buf->ptr+1) != '\n')
	{
		(buf->ptr)++;
		distance++;
	}
	return distance;
}
