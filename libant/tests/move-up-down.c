#include "../libant.h"

int main()
{
	libant_buffer buffer;
	libant_load_buffer_from_file_path("test.txt", &buffer);
	printf("%d\n", libant_move_down(164, &buffer));
	printf("%d\n", libant_move_up(182, &buffer));
}
