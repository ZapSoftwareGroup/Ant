#include "../libant.h"

int main()
{
	char* buffer = NULL;
	libant_load_buffer_from_file_path("test.txt", &buffer);
	if(!buffer)
		printf("Buffer not correctly loaded");
	else
		printf("%s", buffer);
	return 0;
}
