project('ElitePathfinder_rs', 'rust',   version: '0.1.0')

gnome = import('gnome')

# Schema
install_data(
    sources: [ 'data/tesnileft.ElitePathFinder_rs.gschema.xml' ],
    install_dir: join_paths(get_option('datadir'),'glib-2.0','schemas'))

# Readme
install_data(
  'README.md',
  install_dir: join_paths(
    get_option('datadir'),
    'doc',
    meson.project_name()
  )
)

gnome.post_install(
  glib_compile_schemas: true
)