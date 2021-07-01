#include "../libant.h"

int main()
{
	libant_buffer buf;
	libant_init_buffer(&buf);
	if (libant_load_config_file("testconfig", &buf) < 0)
		return -1;
	printf("Options: \n");
	for(int i = 0; i < sizeof(default_settings)/sizeof(struct libant_setting); ++i)
		printf("%s: %s\n", buf.settings[i].name, buf.settings[i].value);
	printf("\n\n");

}
