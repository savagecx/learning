import configparser


class ToxIniParser:
    def __init__(self, ini_file):
        """Use configparser to load ini_file into self.config"""
        self.config = configparser.ConfigParser()
        self.config.read(ini_file)

    @property
    def number_of_sections(self) -> int:
        """Return the number of sections in the ini file.
        New to properties? -> https://pybit.es/articles/property-decorator
        """
        return len(self.config.sections())

    @property
    def environments(self) -> list[str]:
        """Return a list of environments
        (= "envlist" attribute of [tox] section)"""
        envlist = self.config["tox"]["envlist"]
        envlist = ",".join(envlist.splitlines())
        return [env.strip() for env in envlist.split(",") if env]

    @property
    def base_python_versions(self) -> list[str]:
        """Return a list of all basepython across the ini file"""
        versions = set()
        for section in self.config.sections():
            if base := self.config[section].get("basepython"):
                versions.add(base)

        return list(versions)
