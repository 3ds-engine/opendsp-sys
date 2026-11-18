#pragma once

// libctru es un requisito previo para los tipos de la 3DS
#include <3ds.h> 

// Headers principales de OpenDSP
#include "vendor/OpenDSP/library/include/opendsp.h"
#include <opendsp/effects.h> // Si vas a exponer efectos como reverb/delay
#include <opendsp/audio.h> // Si vas a exponer la API de streaming