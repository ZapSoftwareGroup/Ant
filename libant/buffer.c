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
