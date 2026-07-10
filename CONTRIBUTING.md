Contributing to Sustaina Dependency Oracle

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Please be respectful and constructive in all interactions. We're building a community for funding open-source dependencies.

## Getting Started

### Prerequisites

- Rust 1.70+
- Node.js 20+
- Git
- Docker (optional)

### Setup

```bash
# Clone the repository
git clone https://github.com/sustaina-protocol/sustaina-dependency-oracle.git
cd sustaina-dependency-oracle

# Install dependencies
make install

# Run tests to verify setup
make test
```

## Development Workflow

### Before Starting

1. **Create an issue** - Discuss what you want to work on
2. **Fork the repository** - Create your own fork
3. **Create a branch** - Use descriptive names: `feature/xyz` or `bugfix/xyz`

### Making Changes

1. **Follow code style** - See Code Style Guide below
2. **Write tests** - Add tests for new functionality
3. **Update docs** - Update relevant documentation
4. **Run checks** - Execute `make check-all` before committing

### Code Style Guide

#### Rust

```rust
// Use doc comments for public items
/// Registers a new identity mapping.
pub fn register_identity(...) -> Result<(), Error> {
    // Implementation
}

// Use meaningful variable names
let oracle_public_key = derive_public_key()?;

// Handle errors explicitly
match operation() {
    Ok(result) => { /* use result */ },
    Err(e) => return Err(e.into()),
}
```

#### TypeScript

```typescript
// Use interfaces for clarity
interface VerifyRequest {
  oidcToken: string;
  repoName: string;
  stellarAddress: string;
}

// Use async/await
async function verifyToken(token: string): Promise<boolean> {
  // Implementation
}

// Use explicit types
const PORT: number = parseInt(process.env.PORT || '3000', 10);
```

#### React

```typescript
// Use 'use client' for client components
'use client';

// Use functional components
export default function MyComponent({ prop }: Props) {
  return <div>{prop}</div>;
}

// Use TypeScript for props
interface Props {
  title: string;
  count: number;
}
```

### Testing Requirements

All new features must include tests:

**Rust:**
```rust
#[test]
fn test_new_functionality() {
    let result = my_function();
    assert!(result.is_ok());
}
```

**TypeScript:**
```typescript
describe('MyFunction', () => {
  it('should work correctly', async () => {
    const result = await myFunction();
    expect(result).toBeDefined();
  });
});
```

Run tests with:
```bash
make test
```

### Documentation Requirements

Update documentation when:
- Adding new features
- Changing existing behavior
- Adding new API endpoints
- Modifying configuration

Relevant files:
- `README.md` - For user-facing changes
- `ARCHITECTURE.md` - For system design changes
- `DEVELOPMENT.md` - For development procedure changes
- Inline code comments - For complex logic

## Commit Guidelines

### Commit Messages

Follow conventional commits format:

```
type(scope): subject

body

footer
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
```
feat(contract): add update_identity function

- Allows owner to change destination address
- Requires authorization from current owner
- Emits event on successful update

Fixes #123
```

```
fix(oracle): handle expired OIDC tokens

Properly reject tokens that have exceeded expiration time
```

### Commit Size

Keep commits atomic and focused:
- One feature or fix per commit
- Small, reviewable commits
- No unrelated changes

## Pull Request Process

### Before Submitting

1. **Test locally** - Run `make check-all`
2. **Update CHANGELOG** - Document your changes
3. **Review your code** - Self-review before submitting
4. **Rebase on main** - Ensure your branch is up to date

### PR Description

Include:
- **Summary** - What does this PR do?
- **Motivation** - Why is this change needed?
- **Testing** - How was this tested?
- **Checklist** - Mark items as complete

Template:
```markdown
## Summary
Brief description of changes

## Motivation
Why this change is needed

## Testing
How this was tested

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Code follows style guide
- [ ] Commits are well-described
```

### Review Process

1. **Automated checks** - Tests and linting must pass
2. **Code review** - At least one approval required
3. **Feedback** - Address review comments
4. **Merge** - Maintainer will merge after approval

## Contributing by Component

### Smart Contract (`contracts/identity-registry/`)

**Guidelines:**
- Maintain backward compatibility
- Add comprehensive documentation
- Include test coverage >80%
- Run `cargo clippy` for linting

**Testing:**
```bash
cd contracts/identity-registry
cargo test
cargo clippy -- -D warnings
```

### Oracle Service (`oracle-service/`)

**Guidelines:**
- Add logging for all significant operations
- Handle errors gracefully
- Validate all inputs
- Include TypeScript strict mode compliance

**Testing:**
```bash
cd oracle-service
npm run type-check
npm run lint
npm run build
```

### CLI Tool (`cli/sustaina-cli/`)

**Guidelines:**
- Support both interactive and programmatic use
- Provide helpful error messages
- Make commands idempotent when possible
- Document all command options

**Testing:**
```bash
cd cli/sustaina-cli
cargo test
cargo clippy -- -D warnings
```

### Frontend (`apps/explorer-ui/`)

**Guidelines:**
- Ensure accessibility (WCAG AA)
- Test on multiple browsers
- Optimize for mobile and desktop
- Use TypeScript strictly

**Testing:**
```bash
cd apps/explorer-ui
npm run type-check
npm run lint
npm run build
```

## Reporting Issues

### Bug Reports

Include:
- Clear description of the bug
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment (OS, versions, etc.)
- Screenshots if applicable

### Feature Requests

Include:
- Clear problem statement
- Proposed solution
- Alternative approaches considered
- Use cases and benefits

## Security Issues

**Do not** open a public issue for security vulnerabilities.

Instead:
1. Email security@drips.network with details
2. Include reproduction steps
3. Allow time for a fix before public disclosure

## Documentation Contributions

### Updating Docs

1. **Clarity** - Write for your intended audience
2. **Accuracy** - Ensure examples work
3. **Completeness** - Cover all scenarios
4. **Formatting** - Use markdown properly

### Adding Examples

Good examples:
- Are runnable as-is
- Use realistic scenarios
- Include expected output
- Explain what's happening

Bad examples:
- Have syntax errors
- Use incomplete code snippets
- Lack context
- Are out of date

## Release Process

Maintainers handle releases:

1. Update version numbers
2. Update changelog
3. Create release tag
4. GitHub Actions builds and publishes
5. Announce release

Contributors can suggest releases by opening an issue.

## Performance Considerations

When contributing:

### Smart Contract

- Minimize storage operations
- Use efficient data structures
- Avoid unnecessary hashing

### Oracle Service

- Cache JWKS to reduce network calls
- Use connection pooling
- Implement rate limiting

### CLI

- Use parallel processing where appropriate
- Stream output for large datasets
- Minimize memory usage

### Frontend

- Use React.memo for expensive components
- Lazy-load heavy features
- Optimize bundle size

## Accessibility Guidelines

For frontend contributions:

- Use semantic HTML
- Provide alt text for images
- Ensure keyboard navigation works
- Maintain sufficient color contrast
- Test with screen readers

## Questions or Need Help?

- Open a discussion on GitHub
- Check existing documentation
- Ask in code review comments
- Email maintainers if needed

## Recognition

Contributors will be:
- Added to CONTRIBUTORS.md
- Thanked in release notes
- Recognized in project documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

For more information, see the Contributing Guide.
