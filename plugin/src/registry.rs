use alloc::vec::Vec;

/// # Registry
///
/// Defines an interface for registering plugins.
///
/// Registry maintains a registry of plug-ins known to the system
/// and provides an interface for base classes to load any plug-ins required
/// to instantiate a subclass of a given type.
///
/// ## Defining a Base Class API
///
/// In order to use this facility you will generally provide a library
/// which defines the API for a plug-in base class.  This API
/// will be sufficient for the application or framework to make use of
/// custom subclasses that will be written by plug-in developers.
///
/// For example, if you have an image processing application, you might
/// want to support plug-ins that implement image filters. You can define
/// a trait (abstract base class for other languages) for image filters
/// that declares the API your application will require image filters to implement.
///
/// People writing custom filters would write a subclass of ImageFilter that
/// overrides the two methods, implementing their own special filtering
/// behavior.
///
/// ### Enabling Plug-in Loading for the Base Class
///
/// In order for ImageFilter to be able to load plug-ins that implement
/// these custom subclasses, it must be registered with the TfType system.
///
/// The ImageFilter base class, as was mentioned earlier, should be made
/// available in a library that the application links with.  This is done
/// so that plug-ins that want to provide ImageFilters can also link with
/// the library allowing them to subclass ImageFilter.
///
/// ### Registering Plug-ins
///
/// A plug-in developer can now write plug-ins with ImageFilter subclasses.
/// Plug-ins can be implemented either as native dynamic libraries (either
/// regular dynamic libraries or framework bundles) or as Python modules.
///
/// Plug-ins must be registered with the registry.  All plugins are
/// registered via RegisterPlugins().  Plug-in Python modules must be
/// directly importable (in other words they must be able to be found in
/// Python's module path.)  Plugins are registered by providing a path or
/// paths to JSON files that describe the location, structure and contents
/// of the plugin.  The standard name for these files in plugInfo.json.
///
/// Typically, the application that hosts plug-ins will locate and register
/// plug-ins at startup.
///
/// The plug-in facility is lazy.  It does not dynamically load code from
/// plug-in bundles until that code is required.
///
/// ### plugInfo.json
///
/// A plugInfo.json file has the following structure:
///
/// ```json
/// {
///     # Comments are allowed and indicated by a hash at the start of a
///     # line or after spaces and tabs.  They continue to the end of line.
///     # Blank lines are okay, too.
///
///     # This is optional.  It may contain any number of strings.
///     #   Paths may be absolute or relative.
///     #   Paths ending with slash have plugInfo.json appended automatically.
///     #   '*' may be used anywhere to match any character except slash.
///     #   '**' may be used anywhere to match any character including slash.
///     "Includes": [
///         "/absolute/path/to/plugInfo.json",
///         "/absolute/path/to/custom.filename",
///         "/absolute/path/to/directory/with/plugInfo/",
///         "relative/path/to/plugInfo.json",
///         "relative/path/to/directory/with/plugInfo/",
///         "glob*/pa*th/*to*/*/plugInfo.json",
///         "recursive/pa**th/**/"
///     ],
///
///     # This is optional.  It may contain any number of objects.
///     "Plugins": [
///         {
///             # Type is required and may be "library", "python" or "resource".
///             "Type": "library",
///
///             # Name is required.  It should be the Python module name,
///             # the shared library name, or a unique resource name.
///             "Name": "myplugin",
///
///             # This gives the path to the plugin as a whole if the plugin
///             # has substructure. For Python it should be the directory
///             # with the __init__.py file. The path is usually relative.
///             # Root is optional and defaults to "." on architectures that
///             # has a filesystem.
///             "Root": ".",
///
///             # LibraryPath is required by Type "library" and unused
///             # otherwise.  It gives the path to the shared library
///             # object, either absolute or relative to Root.
///             "LibraryPath": "libmyplugin.so",
///
///             # ResourcePath is option.  It defaults to ".".
///             # This gives the path to the plugin's resources directory.
///             # The path is either absolute or relative to Root.
///             "ResourcePath": "resources",
///
///             # Info is required.  It's described below.
///             "Info": {
///                 # Plugin contents.
///             }
///         }
///     ]
/// }
/// ```
///
/// As a special case, if a plugInfo.json contains an object that doesn't
/// have either the "Includes" or "Plugins" keys then it's as if the object
/// was in a "Plugins" array.
///
/// ### Advertising a Plug-in's Contents
///
/// Once the plug-ins are registered, the plug-in facility must also be
/// able to tell what they contain.  Specifically, it must be able to find
/// out what subclasses of what plug-in base classes each plug-in contains.
/// Plug-ins must advertise this information through their plugInfo.json file
/// in the "Info" key.  In the "Info" object there should be a key "Types"
/// holding an object.
///
/// This "Types" object's keys are names of subclasses and its values are yet
/// more objects (the subclass meta-data objects).  The meta-data objects can
/// contain arbitrary key-value pairs. The plug-in mechanism will look for a
/// meta-data key called "displayName" whose value should be the display name
/// of the subclass.  The plug-in mechanism will look for a meta-data key
/// called "bases" whose value should be an array of base class type names.
///
/// For example, a bundle that contains a subclass of ImageFilter might have
/// a plugInfo.json that looks like the following example.
///
/// ```json
/// {
///     "Types": {
///         "MyCustomCoolFilter": {
///             "bases": ["ImageFilter"],
///             "displayName": "Add Coolness to Image"
///             # other arbitrary metadata for MyCustomCoolFilter here
///         }
///     }
/// }
/// ```
///
/// What this says is that the plug-in contains a type called MyCustomCoolFilter
/// which has a base class ImageFilter and that this subclass should be called
/// "Add Coolness to Image" in user-visible contexts.
///
/// In addition to the "displayName" meta-data key which is actually
/// known to the plug-in facility, you may put whatever other information
/// you want into a class' meta-data dictionary.  If your plug-in base class
/// wants to define custom keys that it requires all subclasses to provide,
/// you can do that.  Or, if a plug-in writer wants to define their own keys
/// that their code will look for at runtime, that is OK as well.
///
/// ### Working with Subclasses of a Plug-in Base Class
///
/// Most code with uses types defined in plug-ins doesn't deal with
/// the Plug API directly.  Instead, the TfType interface is used
/// to lookup types and to manufacture instances.  The TfType interface
/// will take care to load any required plugins.
///
/// To wrap up our example, the application that wants to actually use
/// ImageFilter plug-ins would probably do a couple of things.  First, it
/// would get a list of available ImageFilters to present to the user.
/// This could be accomplished as shown in
/// \ref plug_cppcode_PlugRegistry2 "Python Code Example 2" (Doxygen only).
///
/// Then, when the user picks a filter from the list, it would manufacture
/// and instance of the filter as shown in
/// \ref plug_cppcode_PlugRegistry3 "Python Code Example 3" (Doxygen only).
///
/// As was mentioned earlier, this plug-in facility tries to be as lazy
/// as possible about loading the code associated with plug-ins.  To that end,
/// loading of a plugin will be deferred until an instance of a type
/// is manufactured which requires the plugin.
///
/// ### Multiple Subclasses of Multiple Plug-in Base Classes
///
/// It is possible for a bundle to implement multiple subclasses
/// for a plug-in base class if desired.  If you want to package half a dozen
/// ImageFilter subclasses in one bundle, that will work fine.  All must
/// be declared in the plugInfo.json.
///
/// It is possible for there to be multiple classes in your
/// application or framework that are plug-in base classes.  Plug-ins that
/// implement subclasses of any of these base classes can all coexist.  And,
/// it is possible to have subclasses of multiple plug-in base classes in the
/// same bundle.
///
/// When putting multiple subclasses (of the same or different base classes)
/// in a bundle, keep in mind that dynamic loading happens for the whole bundle
/// the first time any subclass is needed, the whole bundle will be loaded.
/// But this is generally not a big concern.
///
/// For example, say the example application also has a plug-in base class
/// "ImageCodec" that allows people to add support for reading and writing
/// other image formats.  Imagine that you want to supply a plug-in that
/// has two codecs and a filter all in a single plug-in.  Your plugInfo.json
/// "Info" object might look something like this example.
///
/// ```json
/// {
///     "Types": {
///         "MyTIFFCodec": {
///             "bases": ["ImageCodec"],
///             "displayName": "TIFF Image"
///         },
///         "MyJPEGCodec": {
///             "bases": ["ImageCodec"],
///             "displayName": "JPEG Image"
///         },
///         "MyCustomCoolFilter" : {
///             "bases": ["ImageFilter"],
///             "displayName": "Add Coolness to Image"
///         }
///     }
/// }
/// ```
///
/// ### Dependencies on Other Plug-ins
///
/// If you write a plug-in that has dependencies on another plug-in that you
/// cannot (or do not want to) link against statically, you can declare
/// the dependencies in your plug-in's plugInfo.json .  A plug-in declares
/// dependencies on other classes with a PluginDependencies key whose value
/// is a dictionary.  The keys of the dictionary are plug-in base class names
/// and the values are arrays of subclass names.
///
/// The following example contains an example of a plug-in that depends on two
/// classes from the plug-in in the previous example.
///
/// ```json
/// {
///     "Types": {
///         "UltraCoolFilter": {
///             "bases": ["MyCustomCoolFilter"],
///             "displayName": "Add Unbelievable Coolness to Image"
///             # A subclass of MyCustomCoolFilter that also uses MyTIFFCodec
///         }
///     },
///     "PluginDependencies": {
///         "ImageFilter": ["MyCustomCoolFilter"],
///         "ImageCodec": ["MyTIFFCodec"]
///     }
/// }
/// ```
///
/// The ImageFilter provided by the plug-in in this example depends on the
/// other ImageFilter MyCoolImageFilter and the ImageCodec MyTIFFCodec.
/// Before loading this plug-in, the plug-in facility will ensure that those
/// two classes are present, loading the plug-in that contains them if needed.
///
/// ### C++ Code Example 1
/// ```cpp
/// // Declare a base class interface
/// class ImageFilter {
///    public:
///    virtual bool CanFilterImage(const ImagePtr & inputImage) = 0;
///    virtual ImagePtr FilterImage(const ImagePtr & inputImage) = 0;
/// };
/// ```
///
/// # Python Code Example 2
/// ```python
/// # Get the names of derived types
/// baseType = Tf.Type.Find(ImageFilter)
/// if baseType:
///     derivedTypes = baseType.GetAllDerived()
///     derivedTypeNames = [ derived.typeName for derived in derivedTypes ]
/// ```
///
/// ### Python Code Example 3
/// ```python
/// # Manufacture an instance of a derived type
/// imageFilterType = Tf.Type.Find(ImageFilter)
/// myFilterType = Tf.Type.FindByName('UltraCoolImageFilter')
/// if myFilterType and myFilterType.IsA(imageFilterType):
///     myFilter = myFilterType.Manufacture()
/// ```
pub struct Registry {}

impl Registry {
    /// Registers all plug-ins discovered at `paths`. Sends
    /// Notice::DidRegisterPlugins with any newly registered plugins.
    pub fn register_plugins(&mut self, paths: &[&str]) {
        todo!()
    }

    /// Retrieve the plugin corresponding to the given `name`. Use this
    /// function if you expect that `name` may name a type provided by a
    /// plugin. Calling this function will incur plugin discovery (but not
    /// loading) if plugin discovery has not yet occurred.
    ///
    /// Note that additional plugins may be registered during program runtime.
    pub fn find_type_by_name(type_name: &str) {
        todo!()
    }

    /// Retrieve the Plugin that derives from `Base` and has the given alias
    /// or type name `type_name`. Use this function if you expect that the
    /// derived type may be provided by a plugin. Calling this function will
    /// incur plugin discovery (but not loading) if plugin discovery has not
    /// yet occurred.
    ///
    /// Note that additional plugins may be registered during program runtime.
    pub fn find_derived_type_by_name<Base>(type_name: &str) -> Base {
        todo!()
    }

    /// Return a vector of types derived directly from `base`. Use this
    /// function if you expect that plugins may provide types derived from
    /// `base`.
    pub fn get_directly_derived_types<Base>() -> Vec<Base> {
        todo!()
    }
}
