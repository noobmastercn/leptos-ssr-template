#[derive(Debug, Copy, Clone, Default)]
pub(crate) enum Theme {
    #[default]
    Ig,
    Twitter,
}

pub(crate) enum CssClass {
    // 首页的样式
    Container,
    // 导航栏的样式
    Nav,
    NavLink,
    // 页脚的样式
    MyFooter,

    // 主页的样式
    HomeContainer,
    HomeH2,
    HomeP,
    HomeButton,

    // 关于页面的样式
    AboutContainer,
    AboutH1,
    AboutP,

    // 404 页面的样式
    ErrorContainer,
    ErrorPageH1,
    ErrorPageP,
    ErrorPageA,
}

impl CssClass {
    pub fn get_css(&self, theme: Theme) -> &'static str {
        match theme {
            Theme::Ig => {
                match self {
                    CssClass::Container => "flex flex-col min-h-screen bg-gradient-to-r from-indigo-100 via-purple-100 to-pink-100",
                    CssClass::Nav => "bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 text-white",
                    CssClass::NavLink => "nav-link font-bold text-lg hover:bg-indigo-700 p-3 rounded-lg transition-all duration-300",
                    CssClass::MyFooter => "mt-auto bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 text-white text-center py-4",
                    CssClass::HomeContainer => "my-0 mx-auto max-w-3xl text-center",
                    CssClass::HomeH2 => "p-6 text-4xl font-bold",
                    CssClass::HomeP => "px-10 pb-10 text-left text-lg",
                    CssClass::HomeButton => "bg-gradient-to-r from-pink-500 to-yellow-500 hover:from-pink-600 hover:to-yellow-600 active:to-pink-700 active:from-yellow-500 px-6 py-3 text-white rounded-full shadow-md transform active:scale-95 transition-all duration-300",
                    CssClass::AboutContainer => "text-center p-10",
                    CssClass::AboutH1 => "text-3xl font-bold my-4",
                    CssClass::AboutP => "mt-2 text-lg",
                    CssClass::ErrorContainer => "m-auto text-center p-20 bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 rounded-lg shadow-xl",
                    CssClass::ErrorPageH1 => "text-9xl font-bold text-white mb-8",
                    CssClass::ErrorPageP => "text-2xl text-white mb-8",
                    CssClass::ErrorPageA => "px-8 py-3 text-lg font-semibold rounded-full bg-white text-indigo-500 hover:bg-gray-200 transition-colors duration-300",
                }
            }
            Theme::Twitter => {
                match self {
                    CssClass::Container => "flex flex-col min-h-screen bg-blue-50",
                    CssClass::Nav => "bg-blue-600 text-white",
                    CssClass::NavLink => "nav-link font-medium text-lg hover:bg-blue-700 p-3 rounded transition-all duration-300",
                    CssClass::MyFooter => "mt-auto bg-blue-600 text-white text-center py-4",
                    CssClass::HomeContainer => "my-0 mx-auto max-w-4xl text-center",
                    CssClass::HomeH2 => "p-6 text-3xl font-bold",
                    CssClass::HomeP => "px-10 pb-10 text-left text-lg",
                    CssClass::HomeButton => "bg-blue-500 hover:bg-blue-600 active:bg-blue-700 px-6 py-3 text-white rounded-full shadow-md transition-all duration-300",
                    CssClass::AboutContainer => "text-center p-10",
                    CssClass::AboutH1 => "text-4xl font-bold my-4",
                    CssClass::AboutP => "mt-2 text-base",
                    CssClass::ErrorContainer => "flex flex-col items-center justify-center h-screen bg-blue-50",
                    CssClass::ErrorPageH1 => "text-6xl font-bold text-blue-600 mb-8",
                    CssClass::ErrorPageP => "text-xl text-blue-600 mb-8",
                    CssClass::ErrorPageA => "px-8 py-3 text-lg font-semibold rounded-full bg-white text-blue-500 hover:bg-gray-200 transition-colors duration-300",
                }
            }
        }
    }
}