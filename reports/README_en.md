<div align="center">
    <img height="200px" src="https://upload.wikimedia.org/wikipedia/commons/8/85/Kharkiv_Polytechnic_Institute.jpg?20150321192409"/>
</div>


# LaTeX Report Template for Students of KhPI

This project is designed for students of KhPI to quickly and easily create reports for various tasks, such as lab work, tests, and other academic work. It automates the report creation process by filling in your personal details and other necessary information into a ready-made report template.

## Requirements

To use this project, you need the following:

### Docker Build (_Recommended_)

- `GNU Make` (for project build automation)
- `Docker` (for building reports in a pre-configured environment)
- `Docker Compose` (to run the Docker container with the necessary parameters)

### Local Build

- `GNU Make` (for project build automation)
- `LaTeX` (with installed base packages like `texlive` or others)

## Configuration

To configure the report for yourself, open the `settings.tex` file and fill in your details. Here's an example of the `settings.tex` content that you need to modify:

```latex
\newcommand{\fullName}{Full Name}
\newcommand{\studyingGroup}{KN-777a}
\newcommand{\variant}{7}

\newcommand{\tacherPosition}{Teacher’s Position}
\newcommand{\tacherFullName}{Teacher’s Full Name}
\newcommand{\subject}{Subject Name}
\newcommand{\department}{Department Name}

\newcommand{\taskName}{Report Title}
% path to your report file, which you can place in the reports folder
\newcommand{\reportMainFile}{templates/lab/main.tex}
```

## Build

After filling in your details and configuring the project for yourself, you can build the report by running the following command:

### Docker Build (_Recommended_)
```bash
make
```

### Local Build
```bash
make build_locale
```

After the build, you will see a **build** folder in the project root, where you can find the final PDF document and other build files.
