#include "../libant.h"

int main()
{
	struct libant_buffer buffer;
	libant_load_buffer_from_file_path("test.txt", &buffer);
	if(!buffer.ptr)
		printf("Buffer not correctly loaded");
	else
		printf("%s", buffer.ptr);
	return 0;
}
