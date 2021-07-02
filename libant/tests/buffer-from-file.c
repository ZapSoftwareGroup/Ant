#include "../libant.h"

int main()
{
	libant_buffer buffer;
	libant_load_buffer_from_file(fopen("test.txt", "r"), &buffer);
	if(!buffer.ptr)
		printf("Buffer not correctly loaded");
	else
		printf("%s", buffer.ptr);
	return 0;
}
