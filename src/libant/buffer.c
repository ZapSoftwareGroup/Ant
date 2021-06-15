/*
 * TODO: Add licence here
 */

#include "buffer.h"

extern int libant_load_buffer_from_file(FILE* fp, char** buffer)
{
	if(fp != NULL)
	{
		// FIXME: Maybe use fstat() for file size instead of fseek()
		if(fseek(fp, 0L, SEEK_END) == 0)
		{
			long bufsize = ftell(fp);
			if(bufsize < 0) { return -1; }

			*buffer = malloc((bufsize + 1)*sizeof(char));
			if(fseek(fp, 0L, SEEK_SET) != 0) { return -1; }

			size_t newLen = fread(*buffer, sizeof(char), bufsize, fp);
			if(ferror(fp) != 0)
				fputs("Error reading file", stderr);
			else
				*(*buffer+newLen++) = '\0';
		}
		fclose(fp);
	}
	return 0;
}
