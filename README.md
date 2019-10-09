# Region_Generator

This is a tool designed by mewo and alby smith for use in modding X4: Foundations.

**** config files are not yet set to reasonable values ***
 
 
Configuration files include:

-- generic config.toml to input variables for region shape, number of regions created, and what fields (asteroids, nebula, lockboxes, etc.) will appear in the region

-- regionconfig.toml to determine default values for each field that may appear in the region; these are modified by the values in the generic config.toml file

-- offsets.toml allows for outputting the connection block needed to insert the region into clusters.xml; currently includes all 117 new sectors created for the Teleportaion Wars (TPWAR) mod, but the program is capable of parsing any number of new sectors from this file *one may be required to avoid errors, even if this portion of the program will not be used
