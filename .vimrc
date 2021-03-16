set expandtab
set backspace=indent,eol,start
set wildmode=longest,list,full
set wildmenu
set number
set cursorline
syntax on

map <C-t><up> :tabr<cr>
map <C-t><down> :tabl<cr>
map <C-t><left> :tabl<cr>
map <C-T><right> :tabn<cr>

autocmd BufRead,BufNewFile *.launch set filetype=xml
autocmd BufRead,BufNewFile *.urdf set filetype=xml
autocmd BufRead,BufNewFile *.xacro set filetype=xml
autocmd BufRead,BufNewFile *.sdf set filetype=xml
autocmd BufRead,BufNewFile *.gazebo set filetype=xml
autocmd BufRead,BufNewFile *.world set filetype=xml

if empty(glob('~/.vim/autoload/plug.vim'))
  silent !curl -fLo ~/.vim/autoload/plug.vim --create-dirs
    \ https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
  autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif

call plug#begin()
Plug 'scrooloose/nerdtree'
Plug 'sickill/vim-monokai'
"Plug 'frazrepo/vim-rainbow'
Plug 'kien/rainbow_parentheses.vim'
Plug 'airblade/vim-gitgutter'
Plug 'tpope/vim-surround'
Plug 'terryma/vim-multiple-cursors'
Plug 'vim-scripts/DoxygenToolkit.vim'
"Plug 'nathanaelkane/vim-indent-guides'
Plug 'preservim/nerdcommenter'
Plug 'vim-airline/vim-airline'
Plug 'junegunn/fzf'
Plug 'tpope/vim-fugitive'
"Plug 'scrooloose/syntastic'
Plug 'ctrlpvim/ctrlp.vim'
Plug 'vim-scripts/taglist.vim'
"Plug 'mileszs/ack.vim'
call plug#end()

let g:rainbow_active = 1
"let g:rainbow_ctermfgs
"let g:indent_guides_enable_on_vim_startup =

colorscheme monokai
