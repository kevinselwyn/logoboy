#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <png.h>

#define LOGO_OFFSET 0x104

static int load_rom(char *romname, unsigned char **rom, size_t *romsize) {
	int rc = 0;
	FILE *romfile = NULL;

	romfile = fopen(romname, "rb");

	if (!romfile) {
		printf("Could not open %s\n", romname);

		rc = 1;
		goto cleanup;
	}

	(void)fseek(romfile, 0, SEEK_END);
	*romsize = (size_t)ftell(romfile);
	(void)fseek(romfile, 0, SEEK_SET);

	if (!romsize) {
		printf("%s is empty\n", romname);

		rc = 1;
		goto cleanup;
	}

	*rom = malloc(sizeof(char) * *romsize + 1);

	if (!*rom) {
		printf("Memory error\n");

		rc = 1;
		goto cleanup;
	}

	if (fread(*rom, 1, *romsize, romfile) != *romsize) {
		printf("Could not read %s\n", romname);

		rc = 1;
		goto cleanup;
	}

cleanup:
	if (romfile) {
		(void)fclose(romfile);
	}

	return rc;
}

static int write_rom(char *romname, unsigned char *rom, size_t romsize) {
	int rc = 0;
	FILE *romfile = NULL;

	romfile = fopen(romname, "w+");

	if (!romfile) {
		printf("Could not open %s\n", romname);

		rc = 1;
		goto cleanup;
	}

	if (fwrite(rom, 1, romsize, romfile) != romsize) {
		printf("Could not write %s\n", romname);

		rc = 1;
		goto cleanup;
	}

cleanup:
	if (romfile) {
		(void)fclose(romfile);
	}

	return rc;
}

static int get_logo(unsigned char *rom, char *filename) {
	int rc = 0, byte = 0, bit = 0, counter = 0;
	int i = 0, j = 0, k = 0, l = 0, x = 0, y = 0;
	int width = 48, height = 8;
	double m = 0.0, n = 0.0;
	unsigned char *data = NULL, *logo = NULL, *new_logo = NULL;
	FILE *file = NULL;
	png_byte color_type = 2, bit_depth = 8;
	png_structp png_ptr;
	png_infop info_ptr;
	png_bytep *row_pointers;

	logo = malloc(sizeof(unsigned char) * 0x30);
	memcpy(logo, rom + LOGO_OFFSET, 0x30);

	new_logo = malloc(sizeof(unsigned char) * 0x30);
	data = malloc(sizeof(unsigned char) * (width * height) + 1);

	counter = 0;

	for (i = 0, j = 2; i < j; i++) {
		for (k = 0, l = 2; k < l; k++) {
			for (m = width * (i * 0.5), n = width * ((i * 0.5) + 0.5); m < n; m += 4) {
				byte = (logo[(int)m + k] & 0xf0) + ((logo[(int)m + 2 + k] & 0xf0) >> 0x04);
				new_logo[counter++] = byte;
			}

			for (m = width * (i * 0.5), n = width * ((i * 0.5) + 0.5); m < n; m += 4) {
				byte = ((logo[(int)m + k] & 0x0f) << 0x04) + (logo[(int)m + 2 + k] & 0x0f);
				new_logo[counter++] = byte;
			}
		}
	}

	counter = 0;

	for (i = 0, j = 0x30; i < j; i++) {
		byte = new_logo[i];

		for (k = 7, l = 0; k >= l; k--) {
			bit = (byte & (int)pow(2, k)) == (int)pow(2, k) ? 1 : 0;
			data[counter++] = 255 - (bit * 255);
		}
	}

	file = fopen(filename, "wb");

	if (!file) {
		printf("Could not open %s\n", filename);

		rc = 1;
		goto cleanup;
	}

	png_ptr = png_create_write_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);

	if (!png_ptr) {
		printf("png_create_write_struct failed\n");

		rc = 1;
		goto cleanup;
	}

	info_ptr = png_create_info_struct(png_ptr);

	if (!info_ptr) {
		printf("png_create_info_struct failed\n");

		rc = 1;
		goto cleanup;
	}

	if (setjmp(png_jmpbuf(png_ptr))) {
		printf("Error initializing write\n");

		rc = 1;
		goto cleanup;
	}

	png_init_io(png_ptr, file);

	if (setjmp(png_jmpbuf(png_ptr))) {
		printf("Error writing header\n");

		rc = 1;
		goto cleanup;
	}

	png_set_IHDR(png_ptr, info_ptr, width, height, bit_depth, color_type, PNG_INTERLACE_NONE, PNG_COMPRESSION_TYPE_BASE, PNG_FILTER_TYPE_BASE);
	png_write_info(png_ptr, info_ptr);

	row_pointers = (png_bytep *)malloc(sizeof(png_bytep) * height + 1);

	for (y = 0; y < height; y++) {
		row_pointers[y] = (png_byte *)malloc(png_get_rowbytes(png_ptr, info_ptr) + 1);
	}

	counter = 0;

	for (y = 0; y < height; y++) {
		png_byte *row = row_pointers[y];

		for (x = 0; x < width; x++) {
			png_byte *rgb = &(row[x * 3]);

			rgb[0] = data[counter];
			rgb[1] = data[counter];
			rgb[2] = data[counter++];
		}
	}

	png_write_image(png_ptr, row_pointers);

	if (setjmp(png_jmpbuf(png_ptr))) {
		printf("Error ending write\n");

		rc = 1;
		goto cleanup;
	}

	png_write_end(png_ptr, NULL);

cleanup:
	if (file) {
		(void)fclose(file);
	}

	if (data) {
		free(data);
	}

	if (logo) {
		free(logo);
	}

	if (new_logo) {
		free(new_logo);
	}

	return rc;
}

static int set_logo(unsigned char **rom, char *filename) {
	int rc = 0, width = 0, height = 0, byte = 0, counter = 0;
	int i = 0, j = 0, k = 0, l = 0, x = 0, y = 0;
	unsigned char *header = NULL, *data = NULL, *logo = NULL, *new_logo = NULL;
	FILE *file = NULL;
	png_byte color_type, bit_depth;
	png_structp png_ptr;
	png_infop info_ptr;
	png_bytep *row_pointers;

	file = fopen(filename, "rb");

	if (!file) {
		printf("Could not open %s\n", filename);

		rc = 1;
		goto cleanup;
	}

	header = malloc(sizeof(char) * 8 + 1);

	if (fread(header, 1, 8, file) != 8) {
		printf("Could not read %s\n", filename);

		rc = 1;
		goto cleanup;
	}

	if (png_sig_cmp(header, 0, 8)) {
		printf("%s is not a PNG\n", filename);

		rc = 1;
		goto cleanup;
	}

	png_ptr = png_create_read_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);

	if (!png_ptr) {
		printf("png_create_read_struct failed\n");

		rc = 1;
		goto cleanup;
	}

	info_ptr = png_create_info_struct(png_ptr);

	if (!info_ptr) {
		printf("png_create_info_struct failed\n");

		rc = 1;
		goto cleanup;
	}

	if (setjmp(png_jmpbuf(png_ptr))) {
		printf("Error initializing read\n");

		rc = 1;
		goto cleanup;
	}

	png_init_io(png_ptr, file);
	png_set_sig_bytes(png_ptr, 8);
	png_read_info(png_ptr, info_ptr);

	width = png_get_image_width(png_ptr, info_ptr);
	height = png_get_image_height(png_ptr, info_ptr);
	color_type = png_get_color_type(png_ptr, info_ptr);
	bit_depth = png_get_bit_depth(png_ptr, info_ptr);

	if (color_type != 2 || bit_depth != 8) {
		printf("PNGs must be 8-bit RGB\n");

		rc = 1;
		goto cleanup;
	}

	row_pointers = (png_bytep*)malloc(sizeof(png_bytep) * height + 1);

	for (y = 0; y < height; y++) {
		row_pointers[y] = (png_byte*)malloc(png_get_rowbytes(png_ptr, info_ptr) + 1);
	}

	png_read_image(png_ptr, row_pointers);

	data = malloc(sizeof(char) * (width * height * 3) + 1);

	for (y = 0, i = 0; y < height; y++) {
		png_byte *row = row_pointers[y];

		for (x = 0; x < width; x++) {
			png_byte *rgb = &(row[x * 3]);

			data[i++] = 1 - (rgb[0] / 255);
		}
	}

	logo = malloc(sizeof(char) * ((width * height) / 8) + 1);
	new_logo = malloc(sizeof(char) * ((width * height) / 8) + 1);

	for (i = 0, j = width * height; i < j; i += 8) {
		byte = 0;

		for (k = 0, l = 8; k < l; k++) {
			byte += data[i + k];

			if (k != l - 1) {
				byte <<= 0x01;
			}
		}

		logo[counter++] = byte;
	}

	counter = 0;

	for (i = 0, j = 2; i < j; i++) {
		for (k = 0, l = 6; k < l; k++) {
			byte = logo[k + (i * (0x30 / 2))] & 0xf0;
			byte += (logo[k + l + (i * (0x30 / 2))] & 0xf0) >> 0x04;
			new_logo[counter++] = byte;

			byte = logo[k + (l * 2) + (i * (0x30 / 2))] & 0xf0;
			byte += (logo[(k + (l * 2)) + l + (i * (0x30 / 2))] & 0xf0) >> 0x04;
			new_logo[counter++] = byte;

			byte = (logo[k + (i * (0x30 / 2))] & 0x0f) << 0x04;
			byte += logo[k + l + (i * (0x30 / 2))] & 0x0f;
			new_logo[counter++] = byte;

			byte = (logo[k + (l * 2) + (i * (0x30 / 2))]) << 0x04;
			byte += logo[(k + (l * 2)) + l + (i * (0x30 / 2))] & 0x0f;
			new_logo[counter++] = byte;
		}
	}

	memcpy(*rom + LOGO_OFFSET, new_logo, 0x30);

cleanup:
	if (header) {
		free(header);
	}

	if (file) {
		(void)fclose(file);
	}

	if (data) {
		free(data);
	}

	if (logo) {
		free(logo);
	}

	if (new_logo) {
		free(new_logo);
	}

	return rc;
}

static void usage(char *exec) {
	int length = 0;

	length = (int)strlen(exec);

	printf("Usage: %s [-g,--get] <rom.gb> <outfile.png>\n", exec);
	printf("%*s [-s,--set] <rom.gb> <infile.png>\n", length + 7, "");
	printf("%*s [-h,--help]\n", length + 7, "");
}

int main(int argc, char *argv[]) {
	int rc = 0;
	size_t romsize = 0;
	char *exec = NULL, *romname = NULL, *action = NULL, *image = NULL;
	unsigned char *rom = NULL;

	exec = argv[0];
	action = argv[1];

	if (argc < 4) {
		usage(exec);

		rc = 1;
		goto cleanup;
	}

	romname = argv[2];
	image = argv[3];

	if (load_rom(romname, &rom, &romsize) != 0) {
		rc = 1;
		goto cleanup;
	}

	if (strcmp(action, "-h") == 0 || strcmp(action, "--help") == 0) {
		usage(exec);

		rc = 1;
		goto cleanup;
	} else if (strcmp(action, "-g") == 0 || strcmp(action, "--get") == 0) {
		get_logo(rom, image);
	} else if (strcmp(action, "-s") == 0 || strcmp(action, "--set") == 0) {
		set_logo(&rom, image);

		if (write_rom(romname, rom, romsize)) {
			rc = 1;
			goto cleanup;
		}
	} else {
		printf("Invalid action: %s\n", action);
	}

cleanup:
	if (rom) {
		free(rom);
	}

	return rc;
}