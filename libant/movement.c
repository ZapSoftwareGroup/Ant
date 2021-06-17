/*
 * TODO: Add licence here
 */

#include "movement.h"

extern int libant_move_down(int pos, struct libant_buffer* buf)
{
	if(buf->ptr[pos] == '\n')
		return pos+1;
	int distance_to_start = libant_get_distance_to_line_start(buf->ptr+pos);
	int newpos = pos + libant_get_distance_to_line_end(buf->ptr+pos);
	for(int i = 0; i < distance_to_start; ++i)
	{
		if(*(buf->ptr+1) && *(buf->ptr+1) != '\n')
			newpos++;
		else
			break;
	}
	return newpos;
}

extern int libant_move_up(int pos, struct libant_buffer* buf)
{
	int distance_to_end = libant_get_distance_to_line_end(buf->ptr+pos);
	int newpos = pos - libant_get_distance_to_line_start(buf->ptr+pos)-1;
	for(int i = 0; i < distance_to_end; ++i)
	{
		if(*(buf->ptr-1) && *(buf->ptr) != '\n')
			newpos--;
		else
			break;
	}
	return newpos;
}

extern int libant_move_left(int pos, struct libant_buffer* buf)
{
	if(*(buf->ptr+(pos-1)) == '\n' || !*(buf->ptr+(pos-1)))
		return pos;
	return pos-1;
}
extern int libant_move_right(int pos, struct libant_buffer* buf)
{
	if(*(buf->ptr+(pos)) == '\n' || !*(buf->ptr+(pos+1)))
		return pos;
	return pos+1;
}

int libant_get_distance_to_line_start(char* buf)
{
	int distance = 0;
	while(*(buf-1) && *(buf-2) != '\n')
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
