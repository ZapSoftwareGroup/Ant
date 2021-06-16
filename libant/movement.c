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
			int distance_to_start = libant_get_distance_to_line_start(buf->ptr);
			buf->ptr += libant_get_distance_to_line_end(buf->ptr);
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
			int distance_to_end = libant_get_distance_to_line_end(buf->ptr);
			buf->ptr -= libant_get_distance_to_line_start(buf->ptr);
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

int libant_get_distance_to_line_start(char* buf)
{
	int distance = 0;
	while(*(buf-1) && *(buf-1) != '\n')
	{
		buf--;
		distance++;
	}
	return distance;
}

int libant_get_distance_to_line_end(char* buf)
{
	int distance = 1;
	if(*buf == '\n')
		return 0;
	while(*(buf+1) && *(buf+1) != '\n')
	{
		buf++;
		distance++;
	}
	return distance;
}

extern char* libant_get_line(int linenum, struct libant_buffer* buf)
{
	char* line = NULL;
	char* bufptr = buf->ptr;
	for(int i = 1; i < linenum; ++i)
	{
		bufptr += libant_get_distance_to_line_end(bufptr)+1;
	}
	line = malloc((libant_get_distance_to_line_end(bufptr)+1)*sizeof(char));
	memcpy(line, bufptr, (libant_get_distance_to_line_end(bufptr))*sizeof(char));
	line[libant_get_distance_to_line_end(bufptr)] = '\0';
	return line;
}
