/*
 * Un commentaire.
 */
// Single line
/* Multi
 * Line
 */

/**
 * Documentation.
 *
 * \ingroup y
 * \class MyNs::Class
 * \brief brief
 *
 * @param x desc
 * @return returns desc
 */

+ - / * | & "" '' ^ !  - % : () {} [] ;

class X {};
enum class X {
    A = 0,
    B,
};
struct x {};

while(false)
{}

if (true)
{}

R"(
lorem
a
    b 
        c
)"

[[nodiscard]]

constexpr std::size_t X;
template <typename Type>;
static_cast<std::size_t>(Class::Max);
dynamic_cast<const Type*>(x);

nullptr;

inline x;
typedef x;
static x;
const x;
auto x;
public : X;
private : X;
using X = Y;
friend X;
virtual void;

namespace MyNS;
namespace MyNS
{
}

// -----

#include <lib.h>
#include "file.h"

class Class : public Base<Class>
{
    friend Base;

    public:
        using X = Y;

        struct Struct;

        Class(Struct s);
        Class(const Class&) = delete;
        Class(Class&&) = delete;
        ~Class();

        Class& operator=(const Class&) = delete;
        Class& operator=(Class&&) = delete;

        virtual void VirtualFn() = 0;
}

Class::~Class()
{
    m_field.Unload();
}

const std::shared_ptr<Type>& Class::GetType() const
{
    return m_type;
}

Type& Class::GeType()
{
    return m_type;
}

std::vector<std::string> Class::GetStrings() const
{
    return {};
}

auto Class::fn() -> MyNS::MyType
{}

auto Class::<Type>::fn(const std::string& str) const -> bool 
{
    return false;
}