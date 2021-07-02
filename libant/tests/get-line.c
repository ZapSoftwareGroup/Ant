#include "../libant.h"


int main()
{
	libant_buffer buffer;
	libant_load_buffer_from_file_path("test.txt", &buffer);
	for(int i = 1; i <= 9; ++i)
	{
		printf("%d: %s\n", i, libant_get_line(i, &buffer));
	}
	return 0;
}
