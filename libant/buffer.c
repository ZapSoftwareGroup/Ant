/*
 * TODO: Add licence here
 */

#include "buffer.h"

extern int libant_load_buffer_from_file(FILE* fp, struct libant_buffer* buf)
{
	if(fp != NULL)
	{
		// FIXME: Maybe use fstat() for file size instead of fseek()
		if(fseek(fp, 0L, SEEK_END) == 0)
		{
			long bufsize = ftell(fp);
			if(bufsize < 0) { return -1; }

			buf->ptr = malloc((bufsize + 1)*sizeof(char));
			if(fseek(fp, 0L, SEEK_SET) != 0) { return -1; }

			size_t newLen = fread(buf->ptr, sizeof(char), bufsize, fp);
			if(ferror(fp) != 0)
				fputs("Error reading file", stderr);
			else
				*(buf->ptr+newLen++) = '\0';
			buf->len = bufsize;
			buf->cur_pos = 0;
			buf->xcoord = buf->ycoord = 1;

		}
		fclose(fp);
	}
	return 0;
}

extern int libant_load_buffer_from_file_path(char* path, struct libant_buffer* buf)
{
	if(libant_load_buffer_from_file(fopen(path, "r"), buf) < 0) { return -1; }
	return 0;
}

extern int libant_buffer_update_coordinates(struct libant_buffer* buf)
{
	if(buf->cur_pos < 0)
		return libant_update_1d_coord(buf);
	else if(buf->xcoord == buf->ycoord && buf->xcoord < 0)
		return libant_update_2d_coords(buf);
	else
		return -1;
}

int libant_update_1d_coord(struct libant_buffer* buf)
{
	int x = buf->xcoord;
	int y = buf->ycoord;
	int pos = 0;
	for(int i = 1; i < y; ++i)
	{
		while(*(buf->ptr+pos) != '\n')
		{
			pos++;
		}
		pos++;
	}
	pos += x;
	if(!*(buf->ptr+pos)) // Sanity check
		return -1;
	buf->cur_pos = pos;
	return 0;
}

int libant_update_2d_coords(struct libant_buffer* buf)
{
	int x = 1;
	int y = 1;
	for(int i = 0; i < buf->cur_pos; ++i)
	{
		// FIXME: Constantly incrementing the x value here seems a like bit
		// of a wastefull approach
		if(*(buf->ptr+i) == '\n')
		{
			y++;
			i++;
			x = 1;
		}
		else
			x++;
	}
	buf->ycoord = y;
	buf->xcoord = x;
	return 0;
}
