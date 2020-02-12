# fix-repository
This project contains the XML schema (XSD) for FIX Repository, the metadata model for FIX specifications.

## Uses
Repository is used to generate
* FIXimate3.0
* FIXML Schema
* Microsoft Word Specification documents

## Version
The latest version of XML schema is called **2010 Edition**. The version of the schema is different than the versions of FIX specifications that it is used to create. Specifications are issued as major FIX versions plus Extension Packs.

## Formats
Each repository version is produced in multiple formats:
* Basic repository: A set of xml files that provides a machine readable representation of each version of a FIX specification.
* Intermediate repository: a basic repository is combined into a single XML file.
* Unified repository: a single XML file is produced for the machine readable portion of one or more basic repositories, e.g. FIX 5.0 plus FIXT 1.1. Machine readable and humanly readable documentation are separated, however.

## License
© Copyright 2015 FIX Protocol Limited

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
