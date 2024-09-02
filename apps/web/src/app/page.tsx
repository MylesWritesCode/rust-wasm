export const metadata = {
  title: 'rust-wasm',
};

export default function Page(): JSX.Element {
  return (
    <div className='container'>
      <h1 className='title'>
        <span>rust-wasm</span>
      </h1>
      <a href='/alert'>alert</a>
      <a href='/metamorph'>metamorph</a>
      <a href='https://github.com/MylesWritesCode/rust-wasm' target='_blank' rel='noreferrer'>
        source
      </a>
    </div>
  );
}
