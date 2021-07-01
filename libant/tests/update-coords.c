#include "../libant.h"


int main()
{
	libant_buffer buffer;
	libant_load_buffer_from_file_path("test.txt", &buffer);
	buffer.cur_pos = 128;
	buffer.xcoord = -1;
	buffer.ycoord = -1;
	libant_buffer_update_coordinates(&buffer);
	printf("ptr_pos:%d: x:%d,y:%d\n", buffer.cur_pos, buffer.xcoord, buffer.ycoord);
	buffer.cur_pos = -1;
	libant_buffer_update_coordinates(&buffer);
	printf("ptr_pos:%d: x:%d,y:%d\n", buffer.cur_pos, buffer.xcoord, buffer.ycoord);
	return 0;
}
